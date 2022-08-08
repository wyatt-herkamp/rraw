pub mod response;

use log::trace;
use reqwest::Body;

use crate::submission::response::SubmissionsResponse;
use crate::submission::SubmissionRetriever;
use crate::{Client, Authorized};

use crate::auth::Authenticator;
use crate::error::Error;
use crate::subreddit::response::{AboutSubreddit, Contributors, Friend, Moderators};
use crate::utils::options::{FeedOption, FriendType};
use async_trait::async_trait;
use serde_json::Value;

/// Subreddit Object
pub struct Subreddit<'a, A: Authenticator> {
    /// Me
    pub(crate) me: &'a Client<A>,
    /// Name
    pub subreddit: AboutSubreddit,
}

impl<'a, A: Authenticator> PartialEq for Subreddit<'a, A> {
    fn eq(&self, other: &Subreddit<A>) -> bool {
        self.subreddit.name == other.subreddit.name
    }
}

impl<'a, A: Authenticator> Subreddit<'a, A> {
    /// Returns a Listing of "Contributors" to the Subreddit
    /// Returns 403 if the contributors are displayed. Most Subreddits will have this disabled
    pub async fn get_contributors(&self, feed: Option<FeedOption>) -> Result<Contributors, Error> {
        let mut string = format!("/r/{}/about/contributors", &self.subreddit);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Contributors>(&*string, true).await;
    }
    /// Returns a Listing of Moderators to the Subreddit
    pub async fn get_moderators(&self, feed: Option<FeedOption>) -> Result<Moderators, Error> {
        let mut string = format!("/r/{}/about/moderators", &self.subreddit);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Moderators>(&*string, true).await;
    }
}

impl<'a, A: Authorized> Subreddit<'a, A> {
    /// Adds a friend to the subreddit
    pub async fn add_friend(&self, username: String, typ: FriendType) -> Result<Friend, Error> {
        trace!(
            "Adding {} to r/{} with type {}",
            &username,
            &self.subreddit,
            &typ
        );
        let string = format!("/r/{}/api/friend", &self.subreddit);

        let body = Body::from(format!("name={}&type={}", username, typ));
        return self.me.post_json::<Friend>(&*string, true, body).await;
    }
    ///  removes a friend from the Subreddit
    pub async fn remove_friend(&self, username: String, typ: FriendType) -> Result<Friend, Error> {
        let string = format!("/r/{}/api/unfriend", &self.subreddit);

        let body = Body::from(format!("name={username}&type={typ}"));
        return self.me.post_json::<Friend>(&*string, true, body).await;
    }

    pub async fn compose(
        &self,
        recipient: String,
        subject: String,
        body: String,
    ) -> Result<Value, Error> {
        let string = format!(
            "api_type=json&subject={subject}&text={body}&to={recipient}&from_sr={}",
            self.subreddit.name
        );
        let body = reqwest::Body::from(string);
        self.me.post_json::<Value>("/api/compose", true, body).await
    }
}
#[async_trait]
impl<'a, A: Authenticator> SubmissionRetriever for Subreddit<'a, A> {
    async fn get_submissions<T: Into<String> + std::marker::Send>(
        &self,
        sort: T,
        feed_options: Option<FeedOption>,
    ) -> Result<SubmissionsResponse, Error> {
        let mut path = format!("/r/{}/{}", &self.subreddit, sort.into());
        if let Some(options) = feed_options {
            options.extend(&mut path)
        }
        return self.me.get_json::<SubmissionsResponse>(&path, false).await;
    }
}
