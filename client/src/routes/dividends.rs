use crate::request::{EmptyResponse, Request, RequestBody};
use chrono::NaiveDate;
use datastore_core::Dividend;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetDividends {
    pub ticker: Option<String>,
    pub dates: Option<(NaiveDate, NaiveDate)>,
}

impl GetDividends {
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

impl Request for GetDividends {
    type Body = ();
    type Response = Vec<Dividend>;

    fn endpoint(&self) -> Cow<str> {
        match (self.ticker.clone(), self.dates) {
            (None, None) => Cow::Borrowed("/dividends"),
            (Some(ticker), None) => Cow::Owned(format!("/dividends/{}", ticker)),
            (Some(ticker), Some((start, end))) => {
                Cow::Owned(format!("/dividends/{}/{}/{}", ticker, start, end))
            }
            _ => todo!(),
        }
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
