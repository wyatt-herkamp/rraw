pub use serde::Deserialize;
use serde_json::Value;

use crate::responses::comments::Comment;
use crate::responses::submission::Submission;
use crate::utils::error::APIError;

pub mod comments;
pub mod other;
pub mod submission;
pub mod subreddit;
pub mod user;

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

impl Listing<GenericResponse<Value>> {
    pub fn safe_children(&self) -> Vec<RedditType> {
        let mut children = Vec::new();
        for x in &self.children {
            children.push(RedditType::get_kind(x.kind.as_str(), x.data.clone()).unwrap())
        }
        children
    }
}

#[derive(Deserialize, Debug)]
pub enum RedditType {
    Comment(Option<Comment>),
    Account,
    Link(Option<Submission>),
    Message,
    Subreddit,
    Award,
}

impl RedditType {
    pub fn get_kind(typ: &str, value: Value) -> Result<RedditType, APIError> {
        match typ {
            "t1" => { Ok(RedditType::Comment(serde_json::from_value(value).unwrap())) }
            "t2" => { Ok(RedditType::Account) }
            "t3" => { Ok(RedditType::Link(serde_json::from_value(value).unwrap())) }
            "t4" => { Ok(RedditType::Message) }
            "t5" => { Ok(RedditType::Subreddit) }
            "t6" => { Ok(RedditType::Award) }
            _ => { Err(APIError::ExpiredToken) }
        }
    }
}