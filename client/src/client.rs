use crate::error::{Error, Result};
use crate::request::{Request, RequestBuilderExt};
use futures::prelude::*;
use reqwest::Client as ReqwestClient;
use std::borrow::Cow;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
pub struct Client<'a> {
    /// The underlying Reqwest client used for requests.
    inner: Arc<ReqwestClient>,
    /// The url to which the request are sent.
    url: Cow<'a, str>,
}

fn env(variable: &str) -> Result<String> {
    env::var(variable).map_err(|e| Error::MissingEnv {
        source: e,
        variable: variable.into(),
    })
}

impl<'a> Client<'a> {
    /// Create a new `Client`.
    pub fn new(url: &'a str) -> Self {
        let inner = Arc::new(ReqwestClient::new());

        Self {
            inner,
            url: Cow::Borrowed(url),
        }
    }

    /// Creates a `Client` from environment variables.
    pub fn from_env() -> Result<Self> {
        let inner = Arc::new(ReqwestClient::new());

        let url = env("DATASTORE_BASE_URL")?;
        Ok(Self {
            inner,
            url: Cow::Owned(url),
        })
    }

    pub async fn send<R: Request>(&self, request: R) -> Result<R::Response> {
        let endpoint = request.endpoint();
        let endpoint = endpoint.trim_matches('/');
        let url = format!("{}/{}", self.url, endpoint);

        let res = self
            .inner
            .request(R::METHOD, &url)
            .headers(request.headers())
            .body_ext(request.body())
            .send()
            .await?;
        let status = res.status();
        if status.is_success() {
            res.json().map_err(From::from).await
        } else if status.is_client_error() {
            Err(Error::ClientError(status, res.text().await?))
        } else {
            Err(Error::ServerError(status, res.text().await?))
        }
    }

    pub async fn send_all<T, R>(&self, requests: T) -> Vec<Result<R::Response>>
    where
        T: Iterator<Item = R>,
        R: Request,
    {
        stream::iter(requests)
            .map(|r| self.send(r).map_into())
            .filter_map(|x| x)
            .collect()
            .await
    }
}
