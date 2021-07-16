use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tokio_postgres::Row;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Dividend {
    pub amount: Decimal,
    pub declared_date: NaiveDate,
    pub ex_date: NaiveDate,
    pub record_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub ticker: String,
}

impl Dividend {
    pub fn new(
        amount: Decimal,
        declared_date: NaiveDate,
        ex_date: NaiveDate,
        record_date: NaiveDate,
        payment_date: NaiveDate,
        ticker: String,
    ) -> Self {
        Self {
            amount,
            declared_date,
            ex_date,
            record_date,
            payment_date,
            ticker,
        }
    }
}

impl TryFrom<Row> for Dividend {
    type Error = tokio_postgres::Error;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: row.try_get("amount")?,
            declared_date: row.try_get("declared_date")?,
            ex_date: row.try_get("ex_date")?,
            record_date: row.try_get("record_date")?,
            payment_date: row.try_get("payment_date")?,
            ticker: row.try_get("ticker")?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Split {
    pub ratio: Decimal,
    pub declared_date: NaiveDate,
    pub ex_date: NaiveDate,
    pub ticker: String,
    pub from_factor: Decimal,
    pub to_factor: Decimal,
}

impl Split {
    pub fn new(
        ratio: Decimal,
        declared_date: NaiveDate,
        ex_date: NaiveDate,
        ticker: String,
        from_factor: Decimal,
        to_factor: Decimal,
    ) -> Self {
        Self {
            ratio,
            declared_date,
            ex_date,
            ticker,
            from_factor,
            to_factor,
        }
    }
}
impl TryFrom<Row> for Split {
    type Error = tokio_postgres::Error;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Self {
            ratio: row.try_get("ratio")?,
            declared_date: row.try_get("declared_date")?,
            ex_date: row.try_get("ex_date")?,
            ticker: row.try_get("ticker")?,
            from_factor: row.try_get("from_factor")?,
            to_factor: row.try_get("to_factor")?,
        })
    }
}
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Aggregate {
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub datetime: DateTime<Utc>,
    pub ticker: String,
}

impl Aggregate {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        open: Decimal,
        high: Decimal,
        low: Decimal,
        close: Decimal,
        volume: Decimal,
        datetime: DateTime<Utc>,
        ticker: String,
    ) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            datetime,
            ticker,
        }
    }
}
impl TryFrom<Row> for Aggregate {
    type Error = tokio_postgres::Error;
    fn try_from(row: Row) -> Result<Self, Self::Error> {
        Ok(Aggregate {
            open: row.try_get("open")?,
            high: row.try_get("high")?,
            low: row.try_get("low")?,
            close: row.try_get("close")?,
            volume: row.try_get("volume")?,
            datetime: row.try_get("datetime")?,
            ticker: row.try_get("ticker")?,
        })
    }
}
