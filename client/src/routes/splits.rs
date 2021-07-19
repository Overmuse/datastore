use crate::request::{EmptyResponse, Request, RequestBody};
use chrono::NaiveDate;
use datastore_core::Split;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetSplits {
    pub ticker: Option<String>,
    pub dates: Option<(NaiveDate, NaiveDate)>,
}

impl GetSplits {
    pub fn new() -> Self {
        Self {
            ticker: None,
            dates: None,
        }
    }

    pub fn ticker<T: ToString>(mut self, ticker: T) -> Self {
        self.ticker = Some(ticker.to_string());
        self
    }

    pub fn dates(mut self, dates: (NaiveDate, NaiveDate)) -> Self {
        self.dates = Some(dates);
        self
    }
}

impl Request for GetSplits {
    type Body = ();
    type Response = Vec<Split>;

    fn endpoint(&self) -> Cow<str> {
        match (self.ticker.clone(), self.dates) {
            (None, None) => Cow::Borrowed("/splits"),
            (Some(ticker), None) => Cow::Owned(format!("/splits/{}", ticker)),
            (Some(ticker), Some((start, end))) => {
                Cow::Owned(format!("/splits/{}/{}/{}", ticker, start, end))
            }
            _ => todo!(),
        }
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
