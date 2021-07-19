use crate::db::{with_db, DbPool};
use crate::handlers;
use warp::Filter;

pub fn backfill_aggregates(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("backfill" / "aggregates" / String)
        .and(warp::post())
        .and(with_db(db))
        .and_then(handlers::aggregates::backfill_aggregates)
}

pub fn backfill_dividends(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("backfill" / "dividends" / String)
        .and(warp::post())
        .and(with_db(db))
        .and_then(handlers::dividends::backfill_dividends)
}

pub fn backfill_splits(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("backfill" / "splits" / String)
        .and(warp::post())
        .and(with_db(db))
        .and_then(handlers::splits::backfill_splits)
}

pub fn backfill_routes(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    backfill_aggregates(db.clone())
        .or(backfill_dividends(db.clone()))
        .or(backfill_splits(db))
}
