pub mod me;
pub mod response;

use crate::auth::Authenticator;
use crate::comments::response::CommentsResponse;
use crate::error::Error;
use crate::responses::listing::RedditListing;
use crate::Client;

use crate::submission::response::SubmissionsResponse;
use crate::user::response::AboutUser;

use crate::utils::options::FeedOption;

/// The User Object for Reddit
pub struct User<'a, A: Authenticator> {
    pub(crate) me: &'a Client<A>,
    pub user: AboutUser,
}

impl<'a, A: Authenticator> PartialEq for User<'a, A> {
    fn eq(&self, other: &User<A>) -> bool {
        self.user.name == other.user.name
    }
}

impl<'a, A: Authenticator> User<'a, A> {
    /// Comments
    pub async fn comments(&self, feed: Option<FeedOption>) -> Result<CommentsResponse, Error> {
        let mut string = format!("/user/{}/comments.json", &self.user);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self
            .me
            .get_json::<CommentsResponse>(&*string, false, false)
            .await;
    }
    /// user Submissions
    pub async fn submissions(
        &self,
        feed: Option<FeedOption>,
    ) -> Result<SubmissionsResponse, Error> {
        let mut string = format!("/user/{}/submitted.json", &self.user);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self
            .me
            .get_json::<SubmissionsResponse>(&*string, false, false)
            .await;
    }
    /// User Overview
    pub async fn overview(&self, feed: Option<FeedOption>) -> Result<RedditListing, Error> {
        let mut string = format!("/user/{}/overview.json", &self.user);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self
            .me
            .get_json::<RedditListing>(&*string, false, false)
            .await;
    }
}
