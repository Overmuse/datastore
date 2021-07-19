use crate::db::DbPool;
use crate::error::Error;
use chrono::NaiveDate;
use datastore_core::Split;
use iex::client::Client;
use iex::splits::GetSplits;
use iex::Range;
use std::convert::TryInto;
use tokio_postgres::types::ToSql;
use warp::reject::{custom, Rejection};

pub async fn get_splits(
    maybe_ticker: Option<String>,
    maybe_start: Option<NaiveDate>,
    maybe_end: Option<NaiveDate>,
    db: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    let ticker;
    let start;
    let end;
    let connection = db.get_connection().await?;
    let mut query = "SELECT * FROM splits".to_string();
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
    let values: Result<Vec<Split>, Error> = connection
        .query(query.as_str(), params.as_slice())
        .await
        .map_err(Error::DbQueryError)?
        .into_iter()
        .map(|v| TryInto::try_into(v).map_err(Error::DbQueryError))
        .collect();
    Ok(warp::reply::json(&values.map_err(custom)?))
}

pub async fn store_split(split: Split, db: DbPool) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    connection.execute(
        "INSERT INTO splits (ratio, declared_date, ex_date, ticker, from_factor, to_factor) VALUES ($1, $2, $3, $4, $5, $6)",
        &[&split.ratio, &split.declared_date, &split.ex_date, &split.ticker, &split.from_factor, &split.to_factor]).await.map_err(Error::DbQueryError)?;
    Ok(warp::reply::reply())
}

pub async fn backfill_splits(ticker: String, db: DbPool) -> Result<impl warp::Reply, Rejection> {
    tokio::spawn(async move {
        let query = GetSplits {
            symbol: &ticker,
            range: Range::FiveYears,
        };
        let client = Client::from_env().unwrap();
        let connection = db.get_connection().await.unwrap();
        let splits = client.send(query).await.unwrap();
        for split in splits {
            connection.execute(
                "INSERT INTO splits (ratio, declared_date, ex_date, ticker, from_factor, to_factor) VALUES ($1, $2, $3, $4, $5, $6)",
                &[&split.ratio, &split.declared_date, &split.ex_date, &ticker, &split.from_factor, &split.to_factor]
            ).await.unwrap();
        }
    });
    Ok(warp::reply::reply())
}
