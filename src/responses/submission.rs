use serde::{Deserialize, Deserializer};


use crate::responses::{GenericListing, GenericResponse};
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


pub type Moderators = GenericListing<Moderator>;

#[derive(Debug, Deserialize)]
pub struct Contributor {
    pub name: String,
    pub id: Option<String>,
    pub rel_id: Option<String>,
    pub date: u64,
}



pub type Contributors = GenericListing<Contributor>;
