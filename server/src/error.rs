use serde::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use tracing::error;
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Debug, Error)]
pub enum Error {
    #[error("error getting connection from DB pool: {0}")]
    DbPoolError(mobc::Error<tokio_postgres::Error>),
    #[error("error executing DB query: {0}")]
    DbQueryError(#[from] tokio_postgres::Error),
    #[error("error migrating database: {0}")]
    DbMigrateError(#[from] refinery::Error),
    #[error("error getting connection from redis pool: {0}")]
    RedisPoolError(mobc::Error<mobc_redis::redis::RedisError>),
    #[error("error executing redis query: {0}")]
    RedisError(#[from] mobc_redis::redis::RedisError),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    error!("{:?}", err);

    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        (StatusCode::BAD_REQUEST, "Invalid Body")
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::DbQueryError(_) => (StatusCode::BAD_REQUEST, "Could not Execute request"),
            _ => {
                error!("unhandled application error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
    } else {
        error!("unhandled error: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    };

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
