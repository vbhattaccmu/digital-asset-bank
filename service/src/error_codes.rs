//! Methods defining error handling for the endpoints.

use std::convert::Infallible;
use warp::{self, http, hyper::StatusCode};

#[derive(Debug)]
pub enum Error {
    ResourceBusy,
    WindowLimitExceeded,
    DatabaseQueryError,
    SerializationFailure,
    NotEnoughBalance,
    SenderDoesNotExist,
    RecipientDoesNotExist,
    AccountExists,
}

impl warp::reject::Reject for Error {}

// handle_rejection receives a `Rejection` and returns a custom error code to the client.
pub(crate) async fn handle_rejection(
    err: warp::reject::Rejection,
) -> Result<impl warp::Reply, Infallible> {
    let (code, message): (StatusCode, &str) = match err.find() {
        Some(Error::WindowLimitExceeded) => (StatusCode::BAD_REQUEST, WINDOW_LIMIT_EXCEEDED),
        Some(Error::SerializationFailure) => (StatusCode::BAD_REQUEST, SERIALIZATION_FAILURE),
        Some(Error::DatabaseQueryError) => (StatusCode::BAD_REQUEST, DB_QUERY_ERROR),
        Some(Error::ResourceBusy) => (StatusCode::INTERNAL_SERVER_ERROR, RESOURCE_BUSY),
        Some(Error::NotEnoughBalance) => (StatusCode::BAD_REQUEST, NOT_ENOUGH_BALANCE),
        Some(Error::SenderDoesNotExist) => (StatusCode::BAD_REQUEST, SENDER_DOES_NOT_EXIST),
        Some(Error::RecipientDoesNotExist) => (StatusCode::BAD_REQUEST, RECEIVER_DOES_NOT_EXIST),
        Some(Error::AccountExists) => (StatusCode::BAD_REQUEST, ACCOUNT_EXISTS),
        None => (StatusCode::INTERNAL_SERVER_ERROR, DB_QUERY_ERROR),
    };

    Ok(http::Response::builder()
        .status(code)
        .body(message.to_string()))
}

const WINDOW_LIMIT_EXCEEDED: &str =
    "Current window limit has exceeded. Please adhere to a max limit of 25.";
const SERIALIZATION_FAILURE: &str = "Serde serialization failed. Please check Tx data stucture.";
const DB_QUERY_ERROR: &str = "Database query error occured. Please check query attributes.";
const RESOURCE_BUSY: &str = "Database resources busy. Please contact system administrator.";
const SENDER_DOES_NOT_EXIST: &str = "Sender id does not exist on record. Please provide correct ID";
const RECEIVER_DOES_NOT_EXIST: &str =
    "Receiver id does not exist on record. Please provide correct ID";
const NOT_ENOUGH_BALANCE: &str =
    "Sender does not have enough balance to submit this transacion.Minimum balance needs to be 5.";
const ACCOUNT_EXISTS: &str = "Account exists on DB";
