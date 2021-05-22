use crate::responses::{GenericListing, GenericResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub link_id: Option<String>,
    pub likes: Option<bool>,
    pub id: Option<String>,
    pub author: Option<String>,
    pub created_utc: Option<f64>,
    pub parent_id: Option<String>,
    pub score: Option<i32>,
    pub author_fullname: Option<String>,
    pub subreddit_id: Option<String>,
    pub body: Option<String>,
    pub link_title: Option<String>,
    pub name: Option<String>,
    pub downs: Option<i32>,
    pub body_html: Option<String>,
    pub distinguished: Option<String>,
    pub stickied: Option<bool>,
    pub ups: Option<i32>,
    pub replies: Option<SubredditReplies>,
}

pub type Reply = GenericListing<Comment>;

#[derive(Debug, Deserialize)]
pub enum SubredditReplies {
    Reply(Reply),
    Str(String),
}

pub type Comments = GenericResponse<Comment>;
