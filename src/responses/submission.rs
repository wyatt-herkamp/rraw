use serde::{Deserialize, Deserializer};

use crate::responses::{GenericListing, GenericResponse};
use crate::responses::subreddit::AboutSubreddit;
use serde::de::Error;

#[derive(Debug, Deserialize)]
pub struct Submission {
    pub domain: String,
    pub subreddit: String,
    pub selftext_html: Option<String>,
    pub selftext: String,
    pub likes: Option<bool>,
    pub id: String,
    pub author: String,
    pub score: f64,
    pub num_comments: u64,
    pub thumbnail: String,
    pub subreddit_id: String,
    pub downs: f64,
    pub ups: f64,
    pub stickied: bool,
    pub locked: bool,
    pub name: String,
    pub created: f64,
    pub url: Option<String>,
    pub title: String,
    pub created_utc: f64,
    pub distinguished: Option<String>,
}

impl<'de> Deserialize<'de> for GenericResponse<Submission> {
    fn deserialize<D>(deserializer: D) -> Result<GenericResponse<Submission>, D::Error> where D: Deserializer<'de>, {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();
        let result = serde_json::from_value(value);
        if let Err(e) = result {
            return Err(D::Error::custom(e.to_string()));
        }
        return Ok(result.unwrap());
    }
}

/// The response from an add friend request
#[derive(Debug, Deserialize)]
pub struct Friend {
    /// Was the friend request a success
    pub success: bool,
}

/// Submissions
pub type Submissions = GenericListing<Submission>;

#[derive(Debug, Deserialize)]
pub struct Moderator {
    pub name: String,
    pub author_flair_text: Option<String>,
    pub author_flair_css_class: Option<String>,
    pub date: u64,
    pub mod_permissions: Vec<String>,
}

impl<'de> Deserialize<'de> for GenericResponse<Moderator> {
    fn deserialize<D>(deserializer: D) -> Result<GenericResponse<Moderator>, D::Error> where D: Deserializer<'de>, {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();
        let result = serde_json::from_value(value);
        if let Err(e) = result {
            return Err(D::Error::custom(e.to_string()));
        }
        return Ok(result.unwrap());
    }
}

pub type Moderators = GenericListing<Moderator>;

#[derive(Debug, Deserialize)]
pub struct Contributor {
    pub name: String,
    pub id: Option<String>,
    pub rel_id: Option<String>,
    pub date: u64,
}

impl<'de> Deserialize<'de> for GenericResponse<Contributor> {
    fn deserialize<D>(deserializer: D) -> Result<GenericResponse<Contributor>, D::Error> where D: Deserializer<'de>, {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();
        let result = serde_json::from_value(value);
        if let Err(e) = result {
            return Err(D::Error::custom(e.to_string()));
        }
        return Ok(result.unwrap());
    }
}

pub type Contributors = GenericListing<Contributor>;