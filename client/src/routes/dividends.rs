use crate::request::Request;
use datastore_core::Dividend;
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
