pub mod comments;
pub mod other;
pub mod submission;
pub mod subreddit;
pub mod user;

pub use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GenericResponse<T> {
    pub kind: String,
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct Listing<T> {
    pub modhash: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<T>,
}

pub type GenericListing<T> = GenericResponse<Listing<GenericResponse<T>>>;
