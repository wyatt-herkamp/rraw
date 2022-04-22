use crate::comments::CommentType;
use crate::responses::listing::GenericListing;
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize)]
pub struct CommentResponse {
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
    pub link_title: Option<String>,
    pub name: Option<String>,
    pub permalink: String,
    pub downs: Option<i32>,
    pub body_html: Option<String>,
    pub distinguished: Option<String>,
    pub stickied: Option<bool>,
    pub ups: Option<i32>,
}
impl Debug for CommentResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Permalink: {}, ID: {}", self.permalink, self.id)
    }
}
impl<'a> CommentType<'a> for CommentResponse {
    fn get_permalink(&self) -> &String {
        &self.permalink
    }
}
pub type CommentsResponse = GenericListing<CommentResponse>;
