use crate::db::{with_db, DbPool};
use crate::handlers;
use warp::Filter;

pub fn list_dividends(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("dividends")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::list_dividends)
}
