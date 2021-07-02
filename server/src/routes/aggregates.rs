use crate::db::{with_db, DbPool};
use crate::handlers;
use warp::Filter;

pub fn list_aggregates(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("aggregates")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::aggregates::list_aggregates)
}

pub fn post_aggregate(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("aggregates")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handlers::aggregates::store_aggregate)
}

pub fn aggregates_routes(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    list_aggregates(db.clone()).or(post_aggregate(db))
}
