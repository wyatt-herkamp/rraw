use serde::de::Error;
pub use serde::Deserialize;
use serde::Deserializer;
use serde_json::Value;

use crate::responses::comments::Comment;
use crate::responses::submission::Submission;
use crate::responses::subreddit::AboutSubreddit;
use crate::responses::user::AboutUser;
use crate::utils::error::APIError;

pub mod comments;
pub mod other;
pub mod submission;
pub mod subreddit;
pub mod user;

#[derive(Debug)]
pub struct GenericResponse<T> {
    pub kind: String,
    pub data: T,
}

pub type RedditResponse = GenericResponse<RedditType>;

impl GenericResponse<RedditType> {
    pub fn new(reddit_type: RedditType) -> GenericResponse<RedditType> {
        let mut kind = match reddit_type {
            RedditType::Comment(_) => "t1",
            RedditType::Account(_) => "t2",
            RedditType::Link(_) => "t3",
            RedditType::Message => "t4",
            RedditType::Subreddit(_) => "t5",
            RedditType::Award => "t6",
        };
        return GenericResponse::<RedditType> {
            kind: kind.to_string(),
            data: reddit_type,
        };
    }
}

impl<'de> Deserialize<'de> for GenericResponse<RedditType> {
    fn deserialize<D>(deserializer: D) -> Result<GenericResponse<RedditType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();

        let x = value["kind"].as_str().unwrap();
        match x {
            "t1" => Ok(GenericResponse::<RedditType>::new(RedditType::Comment(
                (serde_json::from_value(value["data"].clone()).unwrap()),
            ))),
            "t2" => Ok(GenericResponse::<RedditType>::new(RedditType::Account(
                (serde_json::from_value(value["data"].clone()).unwrap()),
            ))),
            "t3" => Ok(GenericResponse::<RedditType>::new(RedditType::Link(
                (serde_json::from_value(value["data"].clone()).unwrap()),
            ))),
            //"t4" => { Ok(GenericResponse::<RedditType>::new(RedditType::Comment(serde_json::from_str(value["data"].as_str().unwrap()).unwrap()))) }
            "t5" => Ok(GenericResponse::<RedditType>::new(RedditType::Subreddit(
                serde_json::from_value(value["data"].clone()).unwrap(),
            ))),
            //"t6" => { Ok(GenericResponse::<RedditType>::new(RedditType::Comment(serde_json::from_str(value["data"].as_str().unwrap()).unwrap()))) }
            _ => Err(D::Error::custom("Invalid Reddit Kind")),
        }
    }
}

impl<'de, T> Deserialize<'de> for GenericListing<T> {
    fn deserialize<D>(deserializer: D) -> Result<GenericListing<T>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();
        let result = serde_json::from_value(value);
        if let Err(e) = result {
            return Err(D::Error::custom(e.to_string()));
        }
        return Ok(result.unwrap());
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
pub type RedditListing = GenericResponse<Listing<GenericResponse<RedditType>>>;

#[derive(Deserialize, Debug)]
pub enum RedditType {
    Comment(Comment),
    Account(AboutUser),
    Link(Submission),
    Message,
    Subreddit(AboutSubreddit),
    Award,
}
