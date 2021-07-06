use crate::request::{EmptyResponse, Request, RequestBody};
use datastore_core::Dividend;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
pub struct GetDividends {}

impl Request for GetDividends {
    type Body = ();
    type Response = Vec<Dividend>;

    fn endpoint(&self) -> Cow<str> {
        Cow::Borrowed("/dividends")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PostDividend(pub Dividend);

impl Request for PostDividend {
    type Body = Dividend;
    type Response = EmptyResponse;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        Cow::Borrowed("/dividends")
    }

    fn body(&self) -> RequestBody<&Self::Body> {
        RequestBody::Json(&self.0)
    }
}
