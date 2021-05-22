pub mod other;
pub mod subreddit;
pub mod user;

pub use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GenericResponse<T> {
    pub kind: String,
    pub data: T,
}