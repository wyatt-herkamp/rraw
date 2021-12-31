use serde::Deserialize;

use crate::responses::GenericListing;

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub link_id: Option<String>,
    pub likes: Option<bool>,
    pub id: String,
    pub author: Option<String>,
    pub created_utc: Option<f64>,
    pub parent_id: Option<String>,
    pub score: f64,
    pub author_fullname: Option<String>,
    pub subreddit_id: Option<String>,
    pub subreddit: String,
    pub body: String,
    pub link_title: String,
    pub name: Option<String>,
    pub permalink: Option<String>,
    pub downs: Option<i32>,
    pub body_html: Option<String>,
    pub distinguished: Option<String>,
    pub stickied: Option<bool>,
    pub ups: Option<i32>,
}

pub type Comments = GenericListing<Comment>;
