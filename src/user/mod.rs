pub mod me;
pub mod response;

use crate::client::Client;
use crate::comments::response::CommentsResponse;
use crate::error::Error;

use crate::responses::RedditListing;
use crate::submission::response::SubmissionsResponse;
use crate::user::response::UserResponse;

use crate::utils::options::FeedOption;

/// The User Object for Reddit
pub struct User<'a> {
    pub(crate) me: &'a Client,
    pub name: String,
}

impl<'a> PartialEq for User<'a> {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name
    }
}

impl<'a> User<'a> {
    /// Gets the about data for the user
    pub async fn about(&self) -> Result<UserResponse, Error> {
        let string = format!("/user/{}/about.json", &self.name);
        return self.me.get_json::<UserResponse>(&*string, false).await;
    }
    /// Comments
    pub async fn comments(&self, feed: Option<FeedOption>) -> Result<CommentsResponse, Error> {
        let mut string = format!("/user/{}/comments.json", &self.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<CommentsResponse>(&*string, false).await;
    }
    /// user Submissions
    pub async fn submissions(
        &self,
        feed: Option<FeedOption>,
    ) -> Result<SubmissionsResponse, Error> {
        let mut string = format!("/user/{}/submitted.json", &self.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self
            .me
            .get_json::<SubmissionsResponse>(&*string, false)
            .await;
    }
    /// User Overview
    pub async fn overview(&self, feed: Option<FeedOption>) -> Result<RedditListing, Error> {
        let mut string = format!("/user/{}/overview.json", &self.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<RedditListing>(&*string, false).await;
    }
}
