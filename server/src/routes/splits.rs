use crate::db::{with_db, DbPool};
use crate::handlers;
use warp::Filter;

pub fn get_splits(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("splits")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::splits::list_splits)
}

pub fn post_split(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("splits")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::splits::store_split)
}

pub fn splits_routes(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_splits(db.clone()).or(post_split(db))
}
