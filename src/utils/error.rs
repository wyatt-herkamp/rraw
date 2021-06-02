use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use reqwest::StatusCode;

#[derive(Debug)]
pub enum APIError {
    ExhaustedListing,

    HTTPError(StatusCode),
    ReqwestError(reqwest::Error),

    JSONError(serde_json::Error),
    ExpiredToken,
    Custom(String),
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Error! {}. ({:?})", self, self)
    }
}

impl Error for APIError {
    fn description(&self) -> &str {
        match self {
            APIError::HTTPError(_) => "The API returned a non-success error code",
            APIError::ReqwestError(_) => "An error occurred while processing the HTTP response",
            APIError::JSONError(_) => {
                "The JSON sent by Reddit did not match what new_rawr was expecting"
            }
            APIError::ExpiredToken => "ExpiredToken",
            APIError::Custom(s) => s.as_str(),
            _ => "This error should not have occurred. Please file a bug",
        }
    }
}

impl From<reqwest::Error> for APIError {
    fn from(err: reqwest::Error) -> APIError {
        APIError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for APIError {
    fn from(err: serde_json::Error) -> APIError {
        APIError::JSONError(err)
    }
}
