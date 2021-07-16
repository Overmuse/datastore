use crate::db::DbPool;
use crate::error::Error;
use chrono::{Local, NaiveDate, TimeZone, Utc};
use core::convert::TryInto;
use datastore_core::Aggregate;
use polygon::rest::{Client, GetAggregate};
use tokio_postgres::types::ToSql;
use warp::reject::{custom, Rejection};

pub async fn get_aggregates(
    ticker: Option<String>,
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    db: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    let tick;
    let s;
    let e;
    let connection = db.get_connection().await?;
    let mut query = "SELECT * FROM daily_aggregates".to_string();
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    if let Some(ticker) = ticker {
        tick = ticker;
        query.push_str(" WHERE ticker = $1");
        params.push(&tick);
    }
    if let Some(start) = start {
        s = Utc.from_utc_datetime(&start.and_hms(0, 0, 0));
        query.push_str(" AND datetime >= $2");
        params.push(&s);
    }
    if let Some(end) = end {
        e = Utc.from_utc_datetime(&end.and_hms(23, 59, 59));
        query.push_str(" AND datetime <= $3");
        params.push(&e);
    }
    query.push_str(";");
    let values: Result<Vec<Aggregate>, Error> = connection
        .query(query.as_str(), params.as_slice())
        .await
        .map_err(Error::DbQueryError)?
        .into_iter()
        .map(|v| TryInto::try_into(v).map_err(Error::DbQueryError))
        .collect();
    Ok(warp::reply::json(&values.map_err(custom)?))
}

pub async fn store_aggregate(
    aggregate: Aggregate,
    db: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    connection.execute(
        "INSERT INTO daily_aggregates (open, high, low, close, volume, datetime, ticker) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&aggregate.open, &aggregate.high, &aggregate.low, &aggregate.close, &aggregate.volume, &aggregate.datetime, &aggregate.ticker]).await.map_err(Error::DbQueryError)?;
    Ok(warp::reply::reply())
}

pub async fn backfill_aggregates(
    ticker: String,
    db: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    tokio::spawn(async move {
        let query = GetAggregate::new(
            &ticker,
            NaiveDate::from_ymd(2010, 1, 1),
            Local::now().naive_local().date(),
        )
        .unadjusted(true)
        .limit(50000);
        let client = Client::from_env().unwrap();
        let connection = db.get_connection().await.unwrap();
        let res = client.send(query).await.unwrap();
        if let Some(aggs) = res.results {
            for aggregate in aggs {
                connection.execute(
                    "INSERT INTO daily_aggregates (open, high, low, close, volume, datetime, ticker) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                    &[&aggregate.o, &aggregate.h, &aggregate.l, &aggregate.c, &aggregate.v, &aggregate.t, &ticker]
                ).await.unwrap();
            }
        }
    });
    Ok(warp::reply::reply())
}
