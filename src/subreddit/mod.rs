pub mod response;

use log::trace;
use reqwest::Body;

use crate::submission::response::SubmissionsResponse;
use crate::submission::SubmissionRetriever;
use crate::Client;

use crate::auth::Authenticator;
use crate::error::Error;
use crate::subreddit::response::{Contributors, Friend, Moderators, SubredditResponse};
use crate::utils::options::{FeedOption, FriendType};
use async_trait::async_trait;

/// Subreddit Object
pub struct Subreddit<'a, A: Authenticator> {
    /// Me
    pub(crate) me: &'a Client<A>,
    /// Name
    pub name: String,
}

impl<'a, A: Authenticator> PartialEq for Subreddit<'a, A> {
    fn eq(&self, other: &Subreddit<A>) -> bool {
        self.name == other.name
    }
}

impl<'a, A: Authenticator> Subreddit<'a, A> {
    ///  Gets the about info the Subreddit
    pub async fn about(&self) -> Result<SubredditResponse, Error> {
        let string = format!("/r/{}/about.json", &self.name);
        return self
            .me
            .get_json::<SubredditResponse>(&*string, self.me.oauth)
            .await;
    }
    /// Gets the contributors for the Subreddit
    pub async fn get_contributors(&self, feed: Option<FeedOption>) -> Result<Contributors, Error> {
        let mut string = format!("/r/{}/about/contributors.json", &self.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Contributors>(&*string, true).await;
    }
    /// Gets a List of Moderators for the subreddit
    pub async fn get_moderators(&self, feed: Option<FeedOption>) -> Result<Moderators, Error> {
        let mut string = format!("/r/{}/about/moderators.json", &self.name);
        if let Some(options) = feed {
            string.push('?');
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Moderators>(&*string, true).await;
    }
    /// Adds a friend to the subreddit
    pub async fn add_friend(&self, username: String, typ: FriendType) -> Result<Friend, Error> {
        trace!(
            "Adding {} to r/{} with type {}",
            &username,
            &self.name,
            &typ
        );
        let string = format!("/r/{}/api/friend", &self.name);

        let body = Body::from(format!("name={}&type={}", username, typ));
        return self.me.post_json::<Friend>(&*string, true, body).await;
    }
    ///  removes a friend from the Subreddit
    pub async fn remove_friend(&self, username: String, typ: FriendType) -> Result<Friend, Error> {
        let string = format!("/r/{}/api/unfriend", &self.name);

        let body = Body::from(format!("name={username}&type={typ}"));
        return self.me.post_json::<Friend>(&*string, true, body).await;
    }
}

#[async_trait]
impl<'a, A: Authenticator> SubmissionRetriever for Subreddit<'a, A> {
    async fn get_submissions<T: Into<String> + std::marker::Send>(
        &self,
        sort: T,
        feed_options: Option<FeedOption>,
    ) -> Result<SubmissionsResponse, Error> {
        let mut path = format!("/r/{}/{}", &self.name, sort.into());
        if let Some(options) = feed_options {
            options.extend(&mut path)
        }
        return self.me.get_json::<SubmissionsResponse>(&path, false).await;
    }
}
