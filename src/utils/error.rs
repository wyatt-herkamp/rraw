use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use reqwest::StatusCode;
use crate::utils::error::APIError::{HTTPError, NotFound};

#[derive(Debug)]
pub enum APIError {
    ExhaustedListing,

    HTTPError(StatusCode),
    NotFound,
    ReqwestError(reqwest::Error),

    JSONError(serde_json::Error),
    ExpiredToken,
    Custom(String),
}

impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        return match self {
            APIError::ExhaustedListing => {
                write!(f, "RRAW ExhaustedListing")
            }
            HTTPError(http) => {
                write!(f, "Http Error! {}", http.as_u16())
            }
            NotFound => {
                write!(f, "Not Found")
            }
            APIError::ExpiredToken => {
                write!(f, "Expired Token")

            }
            APIError::Custom(c) => {
                write!(f, "RRAW Internal Error! {}", c)
            }
            c => {
                write!(f, "RRAW Internal Error! {}", c)
            }
        };
    }
}

impl Error for APIError {}

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
