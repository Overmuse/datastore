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
        .or(list_splits(db))
        .recover(handle_rejection)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "request",
                method = %info.method(),
                path = %info.path()
            )
        }))
}
