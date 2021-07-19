use crate::db::{with_db, DbPool};
use crate::handlers;
use chrono::NaiveDate;
use warp::Filter;

pub fn get_splits(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("splits")
        .and(warp::get())
        .and(with_db(db))
        .and_then(|db| handlers::splits::get_splits(None, None, None, db))
}

pub fn get_splits_with_ticker(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("splits" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|ticker, db| handlers::splits::get_splits(Some(ticker), None, None, db))
}

pub fn get_splits_with_ticker_and_dates(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("splits" / String / NaiveDate / NaiveDate)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|ticker, start, end, db| {
            handlers::splits::get_splits(Some(ticker), Some(start), Some(end), db)
        })
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
    get_splits(db.clone())
        .or(get_splits_with_ticker(db.clone()))
        .or(get_splits_with_ticker_and_dates(db.clone()))
        .or(post_split(db))
}
