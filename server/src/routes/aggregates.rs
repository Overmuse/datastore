use crate::db::{with_db, DbPool};
use crate::handlers;
use chrono::NaiveDate;
use warp::Filter;

pub fn get_aggregates(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("aggregates")
        .and(warp::get())
        .and(with_db(db))
        .and_then(|db| handlers::aggregates::get_aggregates(None, None, None, db))
}

pub fn get_aggregates_with_ticker(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("aggregates" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|ticker, db| handlers::aggregates::get_aggregates(Some(ticker), None, None, db))
}

pub fn get_aggregates_with_ticker_and_dates(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("aggregates" / String / NaiveDate / NaiveDate)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|ticker, start, end, db| {
            handlers::aggregates::get_aggregates(Some(ticker), Some(start), Some(end), db)
        })
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
    get_aggregates(db.clone())
        .or(get_aggregates_with_ticker(db.clone()))
        .or(get_aggregates_with_ticker_and_dates(db.clone()))
        .or(post_aggregate(db))
}
