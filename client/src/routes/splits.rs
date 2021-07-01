use crate::request::Request;
use datastore_core::Split;
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
