use crate::me::Me;
use crate::responses::comments::Comments;
use crate::responses::submission::Submissions;
use crate::responses::user::About;
use crate::responses::{GenericListing, RedditType};
use crate::utils::error::APIError;
use crate::utils::options::FeedOption;
use serde_json::Value;

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
    pub async fn about(&self) -> Result<About, APIError> {
        let string = format!("/u/{}/about.json", self.name.clone());
        return self.me.get_json::<About>(&*string, false).await;
    }
    pub async fn comments(&self, feed: Option<FeedOption>) -> Result<Comments, APIError> {
        let mut string = format!("/u/{}/comments.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Comments>(&*string, false).await;
    }
    pub async fn submissions(&self, feed: Option<FeedOption>) -> Result<Submissions, APIError> {
        let mut string = format!("/u/{}/submitted.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Submissions>(&*string, false).await;
    }
    pub async fn overview(
        &self,
        feed: Option<FeedOption>,
    ) -> Result<GenericListing<Value>, APIError> {
        let mut string = format!("/u/{}/overview.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self
            .me
            .get_json::<GenericListing<Value>>(&*string, false)
            .await;
    }
    pub async fn saved(&self, feed: Option<FeedOption>) -> Result<GenericListing<RedditType>, APIError> {
        let mut string = format!("/u/{}/saved.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self
            .me
            .get_json::<GenericListing<RedditType>>(&*string, false)
            .await;
    }
}
