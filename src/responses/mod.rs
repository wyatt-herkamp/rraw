use serde::de::Error;
pub use serde::Deserialize;
use serde::Deserializer;

use crate::responses::comments::Comment;
use crate::responses::submission::Submission;
use crate::responses::subreddit::AboutSubreddit;
use crate::responses::user::AboutUser;

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
pub struct RedditResponse {
    pub kind: String,
    pub data: RedditType,
}



impl RedditResponse {
    pub fn new(reddit_type: RedditType) ->RedditResponse {
        let kind = match reddit_type {
            RedditType::Comment(_) => "t1",
            RedditType::Account(_) => "t2",
            RedditType::Link(_) => "t3",
            RedditType::Message => "t4",
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
            //"t4" => { Ok(GenericResponse::new(RedditType::Comment(serde_json::from_str(value["data"].as_str().unwrap()).unwrap()))) }
            "t5" => Ok(RedditResponse::new(RedditType::Subreddit(
                serde_json::from_value(value["data"].clone()).unwrap(),
            ))),
            //"t6" => { Ok(GenericResponse::new(RedditType::Comment(serde_json::from_str(value["data"].as_str().unwrap()).unwrap()))) }
            _ => Err(D::Error::custom("Invalid Reddit Kind")),
        }
    }
}


#[derive(Deserialize, Debug)]
pub struct Listing<T> {
    pub modhash: Option<String>,
    pub after: Option<String>,
    pub before: Option<String>,
    pub children: Vec<T>,
}

pub type GenericListing<T> = GenericResponse<Listing<GenericResponse<T>>>;
pub type RedditListing = GenericResponse<Listing<RedditResponse>>;

#[derive( Debug)]
pub enum RedditType {
    Comment(Comment),
    Account(AboutUser),
    Link(Submission),
    Message,
    Subreddit(AboutSubreddit),
    Award,
}
