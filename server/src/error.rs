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
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;
    error!("{:?}", err);

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "Not Found";
    } else if err
        .find::<warp::filters::body::BodyDeserializeError>()
        .is_some()
    {
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::DbQueryError(_) => {
                code = StatusCode::BAD_REQUEST;
                message = "Could not Execute request";
            }
            _ => {
                eprintln!("unhandled application error: {:?}", err);
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Internal Server Error";
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server Error";
    }

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
    });

    Ok(warp::reply::with_status(json, code))
}
