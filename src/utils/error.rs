use crate::utils::error::APIError::{HTTPError, NotFound};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum APIError {
    #[error("The Listing has no more data")]
    ExhaustedListing,
    #[error("HTTP Error Code '{0}'")]
    HTTPError(StatusCode),
    #[error("The requested value is not found")]
    NotFound,
    #[error("Reqwest had an Error {0}")]
    ReqwestError(reqwest::Error),
    #[error("Serde Json Parse Error {0}")]
    JSONError(serde_json::Error),
    #[error("The Token has expired")]
    ExpiredToken,
    #[error("Internal Error {0}")]
    Custom(String),
}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> APIError {
        APIError::ReqwestError(err)
    }
}

impl From<StatusCode> for APIError {
    fn from(err: StatusCode) -> APIError {
        match err {
            StatusCode::NOT_FOUND => {
                return NotFound;
            }
            value => {
                return HTTPError(value);
            }
        }
    }
}

impl From<serde_json::Error> for APIError {
    fn from(err: serde_json::Error) -> APIError {
        APIError::JSONError(err)
    }
}
