use crate::me::Me;
use crate::responses::comments::Comments;
use crate::responses::submission::Submissions;
use crate::responses::user::UserResponse;
use crate::responses::{GenericListing, RedditListing, RedditType};
use crate::utils::error::APIError;
use crate::utils::options::FeedOption;

/// The User Object for Reddit
pub struct User<'a> {
    pub(crate) me: &'a Me,
    pub name: String,
}

impl<'a> PartialEq for User<'a> {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name
    }
}

impl<'a> User<'a> {
    /// Gets the about data for the user
    pub async fn about(&self) -> Result<UserResponse, APIError> {
        let string = format!("/user/{}/about.json", self.name.clone());
        return self.me.get_json::<UserResponse>(&*string, false).await;
    }
    /// Comments
    pub async fn comments(&self, feed: Option<FeedOption>) -> Result<Comments, APIError> {
        let mut string = format!("/user/{}/comments.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Comments>(&*string, false).await;
    }
    /// user Submissions
    pub async fn submissions(&self, feed: Option<FeedOption>) -> Result<Submissions, APIError> {
        let mut string = format!("/user/{}/submitted.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Submissions>(&*string, false).await;
    }
    /// User Overview
    pub async fn overview(&self, feed: Option<FeedOption>) -> Result<RedditListing, APIError> {
        let mut string = format!("/user/{}/overview.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<RedditListing>(&*string, false).await;
    }
    /// Get User saved post. The user must be logged in
    pub async fn saved(&self, feed: Option<FeedOption>) -> Result<RedditListing, APIError> {
        let mut string = format!("/user/{}/saved.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<RedditListing>(&*string, true).await;
    }
}
