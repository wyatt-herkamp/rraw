use crate::auth::Authorized;
use crate::comments::response::CommentsResponse;
use crate::error::Error;
use crate::message::response::MessageListing;
use crate::message::WhereMessage;
use crate::responses::FullName;
use crate::Client;
use serde_json::Value;

use crate::responses::listing::RedditListing;
use crate::submission::response::SubmissionsResponse;
use crate::subreddit::response::Friend;
use crate::user::response::MeResponse;

use crate::utils::options::FeedOption;

/// The User Object for Reddit
pub struct Me<'a, A: Authorized> {
    pub(crate) client: &'a Client<A>,
    pub me: MeResponse,
}

impl<'a, A: Authorized> Me<'a, A> {
    /// For blocking the author of a thing via inbox. - Reddit API
    pub async fn block_author(&self, full_name: FullName) -> Result<Friend, Error> {
        let string = "/api/block";

        let string1 = format!("id={}", full_name);
        let body = reqwest::Body::from(string1);

        self.client.post_json::<Friend>(&string, true, body).await
    }
    /// Gets the Messages. Default for where_message is Inbox
    pub async fn get_messages(
        &self,
        where_message: Option<WhereMessage>,
        feed: Option<FeedOption>,
    ) -> Result<MessageListing, Error> {
        let mut string = format!("/message/{}", where_message.unwrap_or(WhereMessage::Inbox));
        if let Some(f) = feed {
            f.extend(&mut string);
        }
        self.client.get_json::<MessageListing>(&string, true).await
    }
    /// Composes a message.
    pub async fn compose(
        &self,
        recipient: String,
        subject: String,
        body: String,
    ) -> Result<Value, Error> {
        let string = format!("api_type=json&subject={subject}&text={body}&to={recipient}");
        let body = reqwest::Body::from(string);
        self.client
            .post_json::<Value>("/api/compose", true, body)
            .await
    }

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
