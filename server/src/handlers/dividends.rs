use crate::db::DbPool;
use crate::error::Error;
use chrono::NaiveDate;
use core::convert::TryInto;
use datastore_core::Dividend;
use iex::client::Client;
use iex::dividends::GetDividends;
use iex::Range;
use tokio_postgres::types::ToSql;
use warp::reject::{custom, Rejection};

pub async fn get_dividends(
    maybe_ticker: Option<String>,
    maybe_start: Option<NaiveDate>,
    maybe_end: Option<NaiveDate>,
    db: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    let ticker;
    let start;
    let end;
    let connection = db.get_connection().await?;
    let mut query = "SELECT * FROM dividends".to_string();
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    if let Some(t) = maybe_ticker {
        ticker = t;
        query.push_str(" WHERE ticker = $1");
        params.push(&ticker);
    }
    if let Some(s) = maybe_start {
        start = s;
        query.push_str(" AND ex_date >= $2");
        params.push(&start);
    }
    if let Some(e) = maybe_end {
        end = e;
        query.push_str(" AND ex_date <= $3");
        params.push(&end);
    }
    query.push_str(";");
    let values: Result<Vec<Dividend>, Error> = connection
        .query(query.as_str(), params.as_slice())
        .await
        .map_err(Error::DbQueryError)?
        .into_iter()
        .map(|v| TryInto::try_into(v).map_err(Error::DbQueryError))
        .collect();
    Ok(warp::reply::json(&values.map_err(custom)?))
}

pub async fn store_dividend(dividend: Dividend, db: DbPool) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    connection.execute(
        "INSERT INTO dividends (amount, declared_date, ex_date, record_date, payment_date, ticker) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&dividend.amount, &dividend.declared_date, &dividend.ex_date, &dividend.record_date, &dividend.payment_date, &dividend.ticker]).await.map_err(Error::DbQueryError)?;
    Ok(warp::reply::reply())
}

pub async fn backfill_dividends(ticker: String, db: DbPool) -> Result<impl warp::Reply, Rejection> {
    tokio::spawn(async move {
        let query = GetDividends {
            symbol: &ticker,
            range: Range::FiveYears,
        };
        let client = Client::from_env().unwrap();
        let connection = db.get_connection().await.unwrap();
        let dividends = client.send(query).await.unwrap();
        for dividend in dividends {
            connection.execute(
                "INSERT INTO dividends (amount, declared_date, ex_date, record_date, payment_date, ticker) VALUES ($1, $2, $3, $4, $5, $6)",
                &[&dividend.amount, &dividend.declared_date, &dividend.ex_date, &dividend.record_date, &dividend.payment_date, &ticker]
            ).await.unwrap();
        }
    });
    Ok(warp::reply::reply())
}
