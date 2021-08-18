use crate::db::DbPool;
use crate::error::Error;
use chrono::{Local, NaiveDate, TimeZone, Utc};
use core::convert::TryInto;
use datastore_core::Aggregate;
use polygon::rest::{Client, GetAggregate};
use tokio_postgres::types::ToSql;
use warp::reject::{custom, Rejection};

pub async fn get_aggregates(
    maybe_ticker: Option<String>,
    maybe_start: Option<NaiveDate>,
    maybe_end: Option<NaiveDate>,
    db: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    let ticker;
    let start;
    let end;
    let connection = db.get_connection().await?;
    let mut query = "SELECT * FROM daily_aggregates".to_string();
    let mut params: Vec<&(dyn ToSql + Sync)> = Vec::new();
    if let Some(t) = maybe_ticker {
        ticker = t;
        query.push_str(" WHERE ticker = $1");
        params.push(&ticker);
    }
    if let Some(s) = maybe_start {
        start = Utc.from_utc_datetime(&s.and_hms(0, 0, 0));
        query.push_str(" AND datetime >= $2");
        params.push(&start);
    }
    if let Some(e) = maybe_end {
        end = Utc.from_utc_datetime(&e.and_hms(23, 59, 59));
        query.push_str(" AND datetime <= $3");
        params.push(&end);
    }
    let values: Result<Vec<Aggregate>, Error> = connection
        .query(query.as_str(), params.as_slice())
        .await
        .map_err(Error::DbQuery)?
        .into_iter()
        .map(|v| TryInto::try_into(v).map_err(Error::DbQuery))
        .collect();
    Ok(warp::reply::json(&values.map_err(custom)?))
}

pub async fn store_aggregate(
    aggregate: Aggregate,
    db: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    connection.execute(
        "INSERT INTO daily_aggregates (open, high, low, close, volume, datetime, ticker) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (datetime, ticker) DO NOTHING",
        &[&aggregate.open, &aggregate.high, &aggregate.low, &aggregate.close, &aggregate.volume, &aggregate.datetime, &aggregate.ticker]).await.map_err(Error::DbQuery)?;
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
                    "INSERT INTO daily_aggregates (open, high, low, close, volume, datetime, ticker) VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (datetime, ticker) DO NOTHING",
                    &[&aggregate.o, &aggregate.h, &aggregate.l, &aggregate.c, &aggregate.v, &aggregate.t, &ticker]
                ).await.unwrap();
            }
        }
    });
    Ok(warp::reply::reply())
}
