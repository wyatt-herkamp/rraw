use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum InternalError {
    #[error("Reqwest had an Error {0}")]
    ReqwestError(reqwest::Error),
    #[error("Serde Json Parse Error {0}")]
    JSONError(serde_json::Error),
    #[error("Internal Error {0}")]
    Custom(String),
}

impl From<reqwest::Error> for InternalError {
    fn from(err: reqwest::Error) -> InternalError {
        InternalError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for InternalError {
    fn from(err: serde_json::Error) -> InternalError {
        InternalError::JSONError(err)
    }
}
