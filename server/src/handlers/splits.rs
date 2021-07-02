use crate::db::DbPool;
use crate::error::Error;
use datastore_core::Split;
use std::convert::TryInto;
use uuid::Uuid;
use warp::reject::{custom, Rejection};

pub async fn list_splits(db: DbPool) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    let values: Result<Vec<Split>, Error> = connection
        .query("SELECT * FROM splits", &[])
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
        "INSERT INTO splits (id, ratio, declared_date, ex_date, ticker, from_factor, to_factor) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[&Uuid::new_v4(), &split.ratio, &split.declared_date, &split.ex_date, &split.ticker, &split.from_factor, &split.to_factor]).await.map_err(Error::DbQueryError)?;
    Ok(warp::reply::reply())
}
