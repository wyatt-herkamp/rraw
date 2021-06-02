use serde::de::Error;
pub use serde::Deserialize;
use serde::Deserializer;

use crate::responses::comments::Comment;
use crate::responses::message::Message;
use crate::responses::submission::Submission;
use crate::responses::subreddit::AboutSubreddit;
use crate::responses::user::AboutUser;

pub mod comments;
mod message;
pub mod other;
pub mod submission;
pub mod subreddit;
pub mod user;

/// A Generic Response from Reddit the type is pre determined by API
#[derive(Deserialize, Debug)]
pub struct GenericResponse<T> {
    /// The kind value from Reddit
    pub kind: String,
    /// Data
    pub data: T,
}

/// A RedditResponse the type is dynamically decided based on the kind data
pub struct RedditResponse {
    /// Kind data
    pub kind: String,
    /// Data response
    pub data: RedditType,
}

impl RedditResponse {
    /// Creates a new Reddit Response for internal use
    pub fn new(reddit_type: RedditType) -> RedditResponse {
        let kind = match reddit_type {
            RedditType::Comment(_) => "t1",
            RedditType::Account(_) => "t2",
            RedditType::Link(_) => "t3",
            RedditType::Message(_) => "t4",
            RedditType::Subreddit(_) => "t5",
            RedditType::Award => "t6",
        };
        return RedditResponse {
            kind: kind.to_string(),
            data: reddit_type,
        };
    }
}

impl<'de> Deserialize<'de> for RedditResponse {
    fn deserialize<D>(deserializer: D) -> Result<RedditResponse, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();

        let x = value["kind"].as_str().unwrap();
        match x {
            "t1" => Ok(RedditResponse::new(RedditType::Comment(
                serde_json::from_value(value["data"].clone()).unwrap(),
            ))),
            "t2" => Ok(RedditResponse::new(RedditType::Account(
                serde_json::from_value(value["data"].clone()).unwrap(),
            ))),
            "t3" => Ok(RedditResponse::new(RedditType::Link(
                serde_json::from_value(value["data"].clone()).unwrap(),
            ))),
            "t4" => Ok(RedditResponse::new(RedditType::Message(
                serde_json::from_value(value["data"].clone()).unwrap(),
            ))),
            "t5" => Ok(RedditResponse::new(RedditType::Subreddit(
                serde_json::from_value(value["data"].clone()).unwrap(),
            ))),
            //"t6" => { Ok(GenericResponse::new(RedditType::Comment(serde_json::from_str(value["data"].as_str().unwrap()).unwrap()))) }
            _ => Err(D::Error::custom("Invalid Reddit Kind")),
        }
    }
}

#[derive(Deserialize, Debug)]
/// The Listing API for async rawr
pub struct Listing<T> {
    /// Modhash from Reddit
    pub modhash: Option<String>,
    /// After from Reddit
    pub after: Option<String>,
    /// before from Reddit
    pub before: Option<String>,
    /// The Children of the post. Either will be a GenericResponse<T> or A RedditResponse
    pub children: Vec<T>,
}

/// GenericListing mixes the GenericResponse and Listing for simplicity
pub type GenericListing<T> = GenericResponse<Listing<GenericResponse<T>>>;
/// RedditListing uses a RedditResponse
pub type RedditListing = GenericResponse<Listing<RedditResponse>>;

/// RedditType allows for dynamic data responses
#[derive(Debug)]
pub enum RedditType {
    /// Comment
    Comment(Comment),
    /// About user
    Account(AboutUser),
    /// Submission
    Link(Submission),
    /// TODO
    Message(Message),
    /// About Subreddit
    Subreddit(AboutSubreddit),
    /// TODO
    Award,
}
