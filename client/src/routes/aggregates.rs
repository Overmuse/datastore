use crate::request::{EmptyResponse, Request, RequestBody};
use datastore_core::Aggregate;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
pub struct GetAggregates {}

impl Request for GetAggregates {
    type Body = ();
    type Response = Vec<Aggregate>;

    fn endpoint(&self) -> Cow<str> {
        Cow::Borrowed("/aggregates")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PostAggregate(pub Aggregate);

impl Request for PostAggregate {
    type Body = Aggregate;
    type Response = EmptyResponse;
    const METHOD: Method = Method::POST;

    fn endpoint(&self) -> Cow<str> {
        Cow::Borrowed("/aggregates")
    }

    fn body(&self) -> RequestBody<&Self::Body> {
        RequestBody::Json(&self.0)
    }
}
