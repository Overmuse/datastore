use crate::Request;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetLast {
    pub ticker: String,
}

impl GetLast {
    pub fn new(ticker: String) -> Self {
        Self { ticker }
    }
}

impl Request for GetLast {
    type Body = ();
    type Response = Option<f64>;

    fn endpoint(&self) -> Cow<str> {
        Cow::Owned(format!("/last/{}", self.ticker))
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetLastOpen {
    pub ticker: String,
}

impl GetLastOpen {
    pub fn new(ticker: String) -> Self {
        Self { ticker }
    }
}

impl Request for GetLastOpen {
    type Body = ();
    type Response = Option<f64>;

    fn endpoint(&self) -> Cow<str> {
        Cow::Owned(format!("/open/{}", self.ticker))
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetLastClose {
    pub ticker: String,
}

impl GetLastClose {
    pub fn new(ticker: String) -> Self {
        Self { ticker }
    }
}

impl Request for GetLastClose {
    type Body = ();
    type Response = Option<f64>;

    fn endpoint(&self) -> Cow<str> {
        Cow::Owned(format!("/close/{}", self.ticker))
    }
}
