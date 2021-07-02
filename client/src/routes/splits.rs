use crate::request::{EmptyResponse, Request, RequestBody};
use datastore_core::Split;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
pub struct GetSplits {}

impl Request for GetSplits {
    type Body = ();
    type Response = Vec<Split>;

    fn endpoint(&self) -> Cow<str> {
        Cow::Borrowed("/splits")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PostSplit(pub Split);

impl Request for PostSplit {
    type Body = Split;
    type Response = EmptyResponse;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        Cow::Borrowed("/splits")
    }

    fn body(&self) -> RequestBody<&Self::Body> {
        RequestBody::Json(&self.0)
    }
}
