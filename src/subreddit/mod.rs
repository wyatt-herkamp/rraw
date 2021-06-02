use reqwest::Body;

use crate::me::Me;
use crate::responses::submission::{Contributors, Friend, Moderators};
use crate::responses::subreddit::SubredditResponse;
use crate::utils::error::APIError;
use crate::utils::options::{FeedOption, FriendType};

/// Subreddit Object
pub struct Subreddit<'a> {
    /// Me
    pub(crate) me: &'a Me,
    /// Name
    pub name: String,
}

impl<'a> PartialEq for Subreddit<'a> {
    fn eq(&self, other: &Subreddit) -> bool {
        self.name == other.name
    }
}

impl<'a> Subreddit<'a> {
    ///  Gets the about info the Subreddit
    pub async fn about(&self) -> Result<SubredditResponse, APIError> {
        let string = format!("/r/{}/about.json", self.name.clone());
        return self.me.get_json::<SubredditResponse>(&*string, self.me.oauth).await;
    }

    /// Gets the contributors for the Subreddit
    pub async fn get_contributors(
        &self,
        feed: Option<FeedOption>,
    ) -> Result<Contributors, APIError> {
        let mut string = format!("/r/{}/about/contributors.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Contributors>(&*string, true).await;
    }
    /// Gets a List of Moderators for the subreddit
    pub async fn get_moderators(&self, feed: Option<FeedOption>) -> Result<Moderators, APIError> {
        let mut string = format!("/r/{}/about/moderators.json", self.name.clone());
        if let Some(options) = feed {
            string.push_str("?");
            string.push_str(options.url().as_str());
        }
        return self.me.get_json::<Moderators>(&*string, true).await;
    }
    /// Adds a friend to the subreddit
    pub async fn add_friend(&self, username: String, typ: FriendType) -> Result<Friend, APIError> {
        let string = format!("/r/{}/api/friend", self.name.clone());

        let body = Body::from(format!("username={}&type={}", username, typ));
        return self.me.post_json::<Friend>(&*string, true, body).await;
    }
    ///  removes a friend from the Subreddit
    pub async fn remove_friend(&self, username: String, typ: FriendType) -> Result<Friend, APIError> {
        let string = format!("/r/{}/api/unfriend", self.name.clone());

        let body = Body::from(format!("username={}&type={}", username, typ));
        return self.me.post_json::<Friend>(&*string, true, body).await;
    }
}
