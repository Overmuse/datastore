use crate::db::{with_db, DbPool};
use crate::handlers;
use chrono::NaiveDate;
use warp::Filter;

pub fn get_dividends(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("dividends")
        .and(warp::get())
        .and(with_db(db))
        .and_then(|db| handlers::dividends::get_dividends(None, None, None, db))
}

pub fn get_dividends_with_ticker(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("dividends" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|ticker, db| handlers::dividends::get_dividends(Some(ticker), None, None, db))
}

pub fn get_dividends_with_ticker_and_dates(
    db: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("dividends" / String / NaiveDate / NaiveDate)
        .and(warp::get())
        .and(with_db(db))
        .and_then(|ticker, start, end, db| {
            handlers::dividends::get_dividends(Some(ticker), Some(start), Some(end), db)
        })
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
    get_dividends(db.clone())
        .or(get_dividends_with_ticker(db.clone()))
        .or(get_dividends_with_ticker_and_dates(db.clone()))
        .or(post_dividend(db))
}
