use crate::auth::PasswordAuthenticator;
use crate::comments::response::CommentsResponse;
use crate::error::Error;
use crate::Client;

use crate::responses::listing::RedditListing;
use crate::submission::response::SubmissionsResponse;
use crate::user::response::MeResponse;

use crate::utils::options::FeedOption;

/// The User Object for Reddit
pub struct Me<'a> {
    pub(crate) client: &'a Client<PasswordAuthenticator>,
    pub me: MeResponse,
}

impl<'a> Me<'a> {
    /// Comments
    pub async fn comments(&self, feed: Option<FeedOption>) -> Result<CommentsResponse, Error> {
        let mut string = format!("/user/{}/comments", &self.me.about.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self
            .client
            .get_json::<CommentsResponse>(&*string, false)
            .await;
    }
    /// user Submissions
    pub async fn submissions(
        &self,
        feed: Option<FeedOption>,
    ) -> Result<SubmissionsResponse, Error> {
        let mut string = format!("/user/{}/submitted", &self.me.about.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self
            .client
            .get_json::<SubmissionsResponse>(&*string, false)
            .await;
    }
    /// User Overview
    pub async fn overview(&self, feed: Option<FeedOption>) -> Result<RedditListing, Error> {
        let mut string = format!("/user/{}/overview", &self.me.about.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.client.get_json::<RedditListing>(&*string, false).await;
    }
    pub async fn saved(&self, feed: Option<FeedOption>) -> Result<RedditListing, Error> {
        let mut string = format!("/user/{}/saved", &self.me.about.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.client.get_json::<RedditListing>(&*string, false).await;
    }
    pub async fn up_voted(&self, feed: Option<FeedOption>) -> Result<RedditListing, Error> {
        let mut string = format!("/user/{}/upvoted", &self.me.about.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.client.get_json::<RedditListing>(&string, false).await;
    }
    pub async fn down_voted(&self, feed: Option<FeedOption>) -> Result<RedditListing, Error> {
        let mut string = format!("/user/{}/downvoted", &self.me.about.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.client.get_json::<RedditListing>(&string, false).await;
    }
}
