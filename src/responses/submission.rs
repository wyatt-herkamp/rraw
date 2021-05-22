use crate::responses::GenericListing;
use serde::Deserialize;


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

/// Submissions
pub type Submissions = GenericListing<Submission>;
