use std::fmt::{Debug, Formatter};
use serde::de::Error;
pub use serde::Deserialize;
use serde::Deserializer;
use crate::comments::response::CommentResponse;

use crate::responses::message::Message;

use crate::responses::user::AboutUser;
use crate::submission::response::SubmissionResponse;
use crate::subreddit::response::AboutSubreddit;

pub mod message;
pub mod other;
pub mod user;

pub type ListingArray = Vec<RedditListing>;

/// A Generic Response from Reddit the type is pre determined by API
#[derive(Deserialize)]
pub struct GenericResponse<T: Debug> {
    /// The kind value from Reddit
    pub kind: String,
    /// Data
    pub data: T,
}

impl<T: Debug> Debug for GenericResponse<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]. {:?}", self.kind, self.data)
    }
}

/// A RedditResponse the type is dynamically decided based on the kind data
pub struct RedditResponse {
    /// Kind data
    pub kind: String,
    /// Data response
    pub data: RedditType,
}

impl Debug for RedditResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]. {:?}", self.kind, self.data)
    }
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
            RedditType::Listing(_) => "Listing"
        };
        RedditResponse {
            kind: kind.to_string(),
            data: reddit_type,
        }
    }
}

impl<'de> Deserialize<'de> for RedditResponse {
    fn deserialize<D>(deserializer: D) -> Result<RedditResponse, D::Error>
        where
            D: Deserializer<'de>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();
        if let Some(kind) = value["kind"].as_str() {
            return match kind {
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
                "Listing" => Ok(RedditResponse::new(RedditType::Listing(
                    serde_json::from_value(value["data"].clone()).unwrap(),
                ))),
                //"t6" => { Ok(GenericResponse::new(RedditType::Comment(serde_json::from_str(value["data"].as_str().unwrap()).unwrap()))) }
                _ => Err(D::Error::custom("Invalid Reddit Kind")),
            };
        }
        Err(serde::de::Error::custom("Some how we are missing a kind tag"))
    }
}

#[derive(Deserialize)]
/// The Listing API for async RRAW
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

impl<T: Debug> Debug for Listing<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Listing] Children Available: {}", self.children.len())
    }
}

/// GenericListing mixes the GenericResponse and Listing for simplicity
pub type GenericListing<T> = GenericResponse<Listing<GenericResponse<T>>>;
/// RedditListing uses a RedditResponse
pub type RedditListing = GenericResponse<Listing<RedditResponse>>;

/// RedditType allows for dynamic data responses
pub enum RedditType {
    Listing(Listing<RedditResponse>),
    /// Comment
    Comment(CommentResponse),
    /// About user
    Account(AboutUser),
    /// Submission
    Link(SubmissionResponse),
    /// TODO
    Message(Box<Message>),
    /// About Subreddit
    Subreddit(Box<AboutSubreddit>),
    /// TODO
    Award,
}

impl Debug for RedditType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RedditType::Listing(data) => {
                write!(f, "{:?}", data)
            }
            RedditType::Comment(data) => {
                write!(f, "{:?}", data)
            }
            RedditType::Account(data) => {
                write!(f, "{:?}", data)
            }
            RedditType::Link(data) => {
                write!(f, "{:?}", data)
            }
            RedditType::Message(data) => {
                write!(f, "{:?}", data)
            }
            RedditType::Subreddit(data) => {
                write!(f, "{:?}", data)
            }
            RedditType::Award => {
                write!(f, "AWARD!")
            }
        }
    }
}
