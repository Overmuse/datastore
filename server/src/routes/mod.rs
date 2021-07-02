use crate::db::DbPool;
use crate::error::handle_rejection;
use std::convert::Infallible;
use warp::Filter;

mod aggregates;
mod dividends;
mod splits;
pub use aggregates::*;
pub use dividends::*;
pub use splits::*;

pub fn routes(db: DbPool) -> impl Filter<Extract = impl warp::Reply, Error = Infallible> + Clone {
    aggregates_routes(db.clone())
        .or(dividends_routes(db.clone()))
        .or(splits_routes(db))
        .recover(handle_rejection)
        .with(warp::trace::request())
}
