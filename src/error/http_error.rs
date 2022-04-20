use reqwest::StatusCode;
use thiserror::Error;

pub trait IntoResult {
    fn into_result(self) -> Result<(), HTTPError>;
}

impl IntoResult for StatusCode {
    fn into_result(self) -> Result<(), HTTPError> {
        if self.is_success() { Ok(()) } else {
            Err(HTTPError::from(self))
        }
    }
}

#[derive(Error, Debug)]
pub enum HTTPError {
    #[error("HTTP Error Code '{0}'")]
    Other(StatusCode),
    #[error("The requested value is not found")]
    NotFound,
}

impl From<StatusCode> for HTTPError {
    fn from(err: StatusCode) -> HTTPError {
        match err {
            StatusCode::NOT_FOUND => {
                HTTPError::NotFound
            }
            value => {
                HTTPError::Other(value)
            }
        }
    }
}

