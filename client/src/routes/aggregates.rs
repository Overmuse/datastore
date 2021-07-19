use crate::request::{EmptyResponse, Request, RequestBody};
use chrono::NaiveDate;
use datastore_core::Aggregate;
use reqwest::Method;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, Clone, Serialize)]
pub struct GetAggregates {
    pub ticker: Option<String>,
    pub dates: Option<(NaiveDate, NaiveDate)>,
}

impl GetAggregates {
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

impl Request for GetAggregates {
    type Body = ();
    type Response = Vec<Aggregate>;

    fn endpoint(&self) -> Cow<str> {
        match (self.ticker.clone(), self.dates) {
            (None, None) => Cow::Borrowed("/aggregates"),
            (Some(ticker), None) => Cow::Owned(format!("/aggregates/{}", ticker)),
            (Some(ticker), Some((start, end))) => {
                Cow::Owned(format!("/aggregates/{}/{}/{}", ticker, start, end))
            }
            _ => todo!(),
        }
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
