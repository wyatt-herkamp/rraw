use crate::comments::response::CommentResponse;
use crate::error::reddit_error::RedditError;
use crate::error::reddit_error::RedditError::InvalidDataType;
use crate::message::response::Message;
use crate::responses::listing::Listing;
use crate::Error;
use serde::de::Error as DeError;
pub use serde::Deserialize;
use serde::Deserializer;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::submission::response::SubmissionResponse;
use crate::subreddit::response::AboutSubreddit;
use crate::user::response::AboutUser;

pub mod listing;

/// A Generic Response from Reddit the type is pre determined by API
/// Data from Reddit usually follows this format
#[derive(Deserialize)]
pub struct GenericResponse<T: Debug> {
    /// The kind value from Reddit
    pub kind: RedditDataType,
    /// The Data that Reddit has responded with
    pub data: T,
}

impl<T: Debug> Debug for GenericResponse<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]. {:?}", self.kind, self.data)
    }
}
/// An Enum To Represent the Different Types of Data Reddit will respond with
pub enum RedditDataType {
    /// Type: `Listing`
    Listing,
    /// Type: `t1`
    Comment,
    /// Type: `t2`
    Account,
    /// Type: `t3`
    Link,
    /// Type: `t4`
    Message,
    /// Type: `t5`
    Subreddit,
    /// Type: `t6`
    Award,
}
impl Display for RedditDataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = match self {
            RedditDataType::Comment => "t1",
            RedditDataType::Account => "t2",
            RedditDataType::Link => "t3",
            RedditDataType::Message => "t4",
            RedditDataType::Subreddit => "t5",
            RedditDataType::Award => "t6",
            RedditDataType::Listing => "Listing",
        };
        write!(f, "{}", data)
    }
}

impl FromStr for RedditDataType {
    type Err = RedditError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t1" => Ok(RedditDataType::Comment),
            "t2" => Ok(RedditDataType::Account),
            "t3" => Ok(RedditDataType::Link),
            "t4" => Ok(RedditDataType::Message),
            "t5" => Ok(RedditDataType::Subreddit),
            "t6" => Ok(RedditDataType::Message),
            "Listing" => Ok(RedditDataType::Listing),
            data => Err(InvalidDataType(data.to_string())),
        }
    }
}
impl<'de> Deserialize<'de> for RedditDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        RedditDataType::from_str(s.as_str()).map_err(DeError::custom)
    }
}
/// A RedditResponse the type is dynamically decided based on the kind data
/// This is primarily used in Listing that return different type of data
pub type RedditResponse = GenericResponse<RedditTypeResponse>;

impl From<RedditTypeResponse> for RedditResponse {
    fn from(reddit_type: RedditTypeResponse) -> Self {
        let kind = match reddit_type {
            RedditTypeResponse::Comment(_) => RedditDataType::Comment,
            RedditTypeResponse::Account(_) => RedditDataType::Account,
            RedditTypeResponse::Link(_) => RedditDataType::Link,
            RedditTypeResponse::Message(_) => RedditDataType::Message,
            RedditTypeResponse::Subreddit(_) => RedditDataType::Subreddit,
            RedditTypeResponse::Award => RedditDataType::Award,
            RedditTypeResponse::Listing(_) => RedditDataType::Listing,
        };
        RedditResponse {
            kind,
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
                "t1" => Ok(RedditResponse::from(RedditTypeResponse::Comment(
                    serde_json::from_value(value["data"].clone()).unwrap(),
                ))),
                "t2" => Ok(RedditResponse::from(RedditTypeResponse::Account(
                    serde_json::from_value(value["data"].clone()).unwrap(),
                ))),
                "t3" => Ok(RedditResponse::from(RedditTypeResponse::Link(
                    serde_json::from_value(value["data"].clone()).unwrap(),
                ))),
                "t4" => Ok(RedditResponse::from(RedditTypeResponse::Message(
                    serde_json::from_value(value["data"].clone()).unwrap(),
                ))),
                "t5" => Ok(RedditResponse::from(RedditTypeResponse::Subreddit(
                    serde_json::from_value(value["data"].clone()).unwrap(),
                ))),
                "Listing" => Ok(RedditResponse::from(RedditTypeResponse::Listing(
                    serde_json::from_value(value["data"].clone()).unwrap(),
                ))),
                //"t6" => { Ok(GenericResponse::new(RedditType::Comment(serde_json::from_str(value["data"].as_str().unwrap()).unwrap()))) }
                _ => Err(DeError::custom("Invalid Reddit Kind")),
            };
        }
        Err(DeError::custom("Some how we are missing a kind tag"))
    }
}
/// FullNames are the {t1,t2,t3,t4,t5,t6}_{id} you see within Reddit API all the time
pub struct FullName {
    pub reddit_type: RedditDataType,
    pub id: String,
}

impl<'de> Deserialize<'de> for FullName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FullName::from_str(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl FromStr for FullName {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('_').collect::<Vec<&str>>();
        if split.len() != 2 {
            // Yes, it is always a good time to make a monty python joke.
            return Err(Error::from("Then shalt thou count to two, no more, no less. Two shall be the number thou shalt count, and the number of the counting shall be two."));
        }
        return Ok(FullName {
            reddit_type: RedditDataType::from_str(split.first().unwrap())?,
            id: split.get(1).unwrap().to_string(),
        });
    }
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.reddit_type, self.id)
    }
}
impl Debug for FullName {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}_{}", self.reddit_type, self.id)
    }
}
/// Reddit Type Response Enum
pub enum RedditTypeResponse {
    /// The Listing Type
    Listing(Listing<RedditResponse>),
    /// Comment Response
    Comment(CommentResponse),
    /// About User Response
    Account(AboutUser),
    /// Submission Type
    Link(SubmissionResponse),
    /// Message Response
    /// Boxed for Memory Safety
    Message(Box<Message>),
    /// About SubReddit Response
    /// Boxed for Memory Safety
    Subreddit(Box<AboutSubreddit>),
    /// TODO
    Award,
}

impl Debug for RedditTypeResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RedditTypeResponse::Listing(data) => {
                write!(f, "{:?}", data)
            }
            RedditTypeResponse::Comment(data) => {
                write!(f, "{:?}", data)
            }
            RedditTypeResponse::Account(data) => {
                write!(f, "{:?}", data)
            }
            RedditTypeResponse::Link(data) => {
                write!(f, "{:?}", data)
            }
            RedditTypeResponse::Message(data) => {
                write!(f, "{:?}", data)
            }
            RedditTypeResponse::Subreddit(data) => {
                write!(f, "{:?}", data)
            }
            RedditTypeResponse::Award => {
                write!(f, "AWARD!")
            }
        }
    }
}
