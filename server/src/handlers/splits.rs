use crate::db::DbPool;
use crate::error::Error;
use core::convert::TryInto;
use datastore_core::Split;
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
