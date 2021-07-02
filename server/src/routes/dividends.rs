use crate::db::{with_db, DbPool};
use crate::handlers;
use warp::Filter;

pub fn list_dividends(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("dividends")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::dividends::list_dividends)
}

pub fn post_dividend(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("dividends")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::dividends::store_dividend)
}

pub fn dividends_routes(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_dividends(db.clone()).or(post_dividend(db))
}
