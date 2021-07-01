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
    list_aggregates(db.clone())
        .or(list_dividends(db.clone()))
        .or(list_splits(db.clone()))
        .recover(handle_rejection)
}
