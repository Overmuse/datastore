use crate::db::DbPool;
use crate::error::Error;
use core::convert::TryInto;
use datastore_core::Aggregate;
use uuid::Uuid;
use warp::reject::{custom, Rejection};

pub async fn list_aggregates(db: DbPool) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    let values: Result<Vec<Aggregate>, Error> = connection
        .query("SELECT * FROM aggregates", &[])
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
        "INSERT INTO aggregates (id, open, high, low, close, volume, start_datetime, end_datetime, ticker) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        &[&Uuid::new_v4(), &aggregate.open, &aggregate.high, &aggregate.low, &aggregate.close, &aggregate.volume, &aggregate.start, &aggregate.end, &aggregate.ticker]).await.map_err(Error::DbQueryError)?;
    Ok(warp::reply::reply())
}
