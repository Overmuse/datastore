use crate::db::DbPool;
use crate::error::Error;
use core::convert::TryInto;
use datastore_core::Dividend;
use warp::reject::{custom, Rejection};

pub async fn list_dividends(db: DbPool) -> Result<impl warp::Reply, Rejection> {
    let connection = db.get_connection().await?;
    let values: Result<Vec<Dividend>, Error> = connection
        .query("SELECT * FROM dividends", &[])
        .await
        .map_err(Error::DBQueryError)?
        .into_iter()
        .map(|v| TryInto::try_into(v).map_err(Error::DBQueryError))
        .collect();
    Ok(warp::reply::json(&values.map_err(custom)?))
}
