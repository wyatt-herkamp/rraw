use core::fmt;
use std::fmt::{Display, Formatter};

use reqwest::Body;
use serde_json::Value;

use crate::me::{FullName, Me};
use crate::responses::RedditListing;
use crate::responses::submission::Friend;
use crate::utils::error::APIError;
use crate::utils::options::FeedOption;

pub struct Inbox<'a> {
    pub(crate) me: &'a Me,
}

impl<'a> Inbox<'a> {
    /// For blocking the author of a thing via inbox. - Reddit API
    pub async fn block_author(&self, full_name: FullName) -> Result<Friend, APIError> {
        let string = "/api/block";

        let string1 = format!("id={}", full_name.to_string());
        let body = Body::from(string1);

        self.me.post_json::<Friend>(&*string, true, body).await
    }
    pub async fn get_messages(&self, where_message: Option<WhereMessage>, feed: Option<FeedOption>) -> Result<RedditListing, APIError> {
        let mut string = format!("/message/{}", where_message.unwrap_or(WhereMessage::Inbox));
        println!("{}", &string);
        if let Some(f) = feed {
            f.extend(&mut string);
        }
        self.me.get_json::<RedditListing>(&*string, true).await
    }
    pub async fn compose(&self, recipient: String, subject: String, body: String, subreddit: Option<String>) -> Result<Value, APIError> {
        let mut string = format!("api_type=json&subject={}&text={}&to={}", subject, body, recipient);
        if let Some(sr) = subreddit {
            string.push_str(format!("&from_sr={}", sr).as_str());
        }
        let body = Body::from(string);
        self.me.post_json::<Value>("/api/compose", true, body).await
    }
}

pub enum WhereMessage {
    Inbox,
    Unread,
    SENT,
}


impl Display for WhereMessage {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            WhereMessage::Inbox => { "inbox" }
            WhereMessage::Unread => { "unread" }
            WhereMessage::SENT => { "sent" }
        };
        write!(f, "{}", string)
    }
}