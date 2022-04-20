use std::fmt::{Debug, Formatter};
use serde::Deserialize;

use crate::responses::GenericListing;
use crate::submission::SubmissionType;

#[derive(Deserialize, Clone)]
pub struct SubmissionResponse {
    pub domain: String,
    pub subreddit: String,
    pub selftext_html: Option<String>,
    pub selftext: String,
    pub likes: Option<bool>,
    pub id: String,
    pub author: String,
    pub score: f64,
    pub num_comments: i64,
    pub thumbnail: String,
    pub subreddit_id: String,
    pub downs: f64,
    pub ups: f64,
    pub stickied: bool,
    pub locked: bool,
    pub over_18: bool,
    pub name: String,
    pub created: f64,
    pub url: Option<String>,
    pub permalink: String,
    pub title: String,
    pub created_utc: f64,
    pub distinguished: Option<String>,
}

impl Debug for SubmissionResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Submission]. Permalink: {}, ID: {}", self.permalink, self.id)
    }
}

impl<'a> SubmissionType<'a> for SubmissionResponse {
    fn get_permalink(&self) -> &String {
        &self.permalink
    }
}

pub type SubmissionsResponse = GenericListing<SubmissionResponse>;



