use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum RedditError {
    #[error("The Data Type specified in not valid {0}")]
    InvalidDataType(String),
}
