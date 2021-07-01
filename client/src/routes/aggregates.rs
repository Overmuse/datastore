use crate::request::Request;
use datastore_core::Aggregate;
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
