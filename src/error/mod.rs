pub mod http_error;
pub mod reddit_error;
pub mod internal_error;

use reqwest::StatusCode;
use thiserror::Error;
use crate::error::http_error::HTTPError;
use crate::error::internal_error::InternalError;
use crate::error::reddit_error::RedditError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Http Error: {0}")]
    HTTPError(HTTPError),
    #[error("Internal Error: {0}")]
    InternalError(InternalError),
    #[error("Reddit Error: {0}")]
    RedditError(RedditError),
    #[error("{0}")]
    Other(String),
}

impl From<RedditError> for Error {
    fn from(value: RedditError) -> Self {
        Error::RedditError(value)
    }
}

impl From<HTTPError> for Error {
    fn from(value: HTTPError) -> Self {
        Error::HTTPError(value)
    }
}

impl From<InternalError> for Error {
    fn from(value: InternalError) -> Self {
        Error::InternalError(value)
    }
}

impl From<StatusCode> for Error {
    fn from(err: StatusCode) -> Error {
        Error::HTTPError(http_error::HTTPError::from(err))
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Error {
        Error::Other(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::InternalError(InternalError::ReqwestError(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        InternalError::JSONError(err).into()
    }
}
