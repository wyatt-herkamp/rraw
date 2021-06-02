use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};

use reqwest::{Body, Client, Response};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{de, Deserialize, Deserializer};
use serde::de::DeserializeOwned;

use crate::auth::Authenticator;
use crate::message::Inbox;
use crate::responses::subreddit::Subreddits;
use crate::responses::user::Users;
use crate::subreddit::Subreddit;
use crate::user::User;
use crate::utils::error::APIError;
use crate::utils::options::FeedOption;

/// This is who you are. This is your identity and you access point to the Reddit API

pub struct Me {
    auth: Arc<Mutex<Box<dyn Authenticator>>>,
    client: Client,
    user_agent: String,
}

impl Me {
    /// Logs into Reddit and Returns a Me
    pub async fn login(
        auth: Arc<Mutex<Box<dyn Authenticator>>>,
        user_agent: String,
    ) -> Result<Me, APIError> {
        let client = Client::new();
        let _x = auth.lock().unwrap().login(&client, &user_agent).await;
        Ok(Me {
            auth,
            client,
            user_agent,
        })
    }
    /// Gets the authenticator. Internal use
    pub fn get_authenticator(&self) -> MutexGuard<Box<dyn Authenticator + 'static>> {
        self.auth.lock().unwrap()
    }
    /// Creates a subreddit object. However, this will not tell you if the user exists.
    pub fn subreddit(&self, name: String) -> Subreddit {
        Subreddit { me: self, name }
    }
    /// Inbox
    pub fn inbox(&self) -> Inbox {
        Inbox { me: self }
    }
    /// Creates a user object. However, this will not tell you if the user exists.
    pub fn user(&self, name: String) -> User {
        User { me: self, name }
    }
    /// Makes a get request with Reqwest response
    pub async fn get(&self, url: &str, oauth: bool) -> Result<Response, reqwest::Error> {
        let mut guard = self.get_authenticator();
        if guard.needs_token_refresh() {
            guard.login(&self.client, self.user_agent.as_str()).await;
        }
        let string = self.build_url(url, oauth);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&*self.user_agent).unwrap(),
        );
        guard.headers(&mut headers);
        self.client.get(string).headers(headers).send().await
    }
    /// Makes a post request with Reqwest response
    pub async fn post(
        &self,
        url: &str,
        oauth: bool,
        body: Body,
    ) -> Result<Response, reqwest::Error> {
        let mut guard = self.get_authenticator();
        if guard.needs_token_refresh() {
            guard.login(&self.client, self.user_agent.as_str()).await;
        }
        let string = self.build_url(url, oauth);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&*self.user_agent).unwrap(),
        );
        guard.headers(&mut headers);
        self.client
            .post(string)
            .body(body)
            .headers(headers)
            .send()
            .await
    }
    /// Makes a get request with JSON response
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        oauth: bool,
    ) -> Result<T, APIError> {
        let x = self.get(url, oauth).await;
        return Me::respond::<T>(x).await;
    }
    /// Makes a post request with JSON response
    pub async fn post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        oauth: bool,
        body: Body,
    ) -> Result<T, APIError> {
        let x = self.post(url, oauth, body).await;
        return Me::respond::<T>(x).await;
    }
    /// Builds a URL
    pub fn build_url(&self, dest: &str, oauth_required: bool) -> String {
        let stem = if oauth_required {
            "https://oauth.reddit.com"
        } else {
            "https://api.reddit.com"
        };
        format!("{}{}", stem, dest)
    }
    /// Handles a Response from Reqwest mainly for internal use
    pub async fn respond<T: DeserializeOwned>(
        result: Result<Response, reqwest::Error>,
    ) -> Result<T, APIError> {
        if let Ok(response) = result {
            let code = response.status();
            if !code.is_success() {
                return Err(APIError::HTTPError(code));
            }

            let value = response.json::<T>().await;
            if let Ok(about) = value {
                return Ok(about);
            } else if let Err(response) = value {
                return Err(APIError::from(response));
            }
        } else if let Err(response) = result {
            return Err(APIError::from(response));
        }
        return Err(APIError::ExhaustedListing);
    }
    /// Searches Reddit for subreddits
    pub async fn search_subreddits(
        &self,
        name: String,
        limit: Option<u64>,
        feed: Option<FeedOption>,
    ) -> Result<Subreddits, APIError> {
        let mut url = format!("https://www.reddit.com/subreddits/search.json?q={}", name);
        if let Some(options) = feed {
            url.push_str(options.url().as_str());
        }
        if let Some(limit) = limit {
            url.push_str(&mut format!("&limit={}", limit));
        }
        self.get_json::<Subreddits>(&*url, false).await
    }
    /// Searches Reddit for Users
    pub async fn search_users(
        &self,
        name: String,
        limit: Option<u64>,
        feed: Option<FeedOption>,
    ) -> Result<Users, APIError> {
        let mut url = format!("https://www.reddit.com/users/search.json?q={}", name);
        if let Some(options) = feed {
            url.push_str(options.url().as_str());
        }
        if let Some(limit) = limit {
            url.push_str(&mut format!("&limit={}", limit));
        }
        self.get_json::<Users>(&*url, false).await
    }
}

#[derive(Debug)]
pub enum RedditType {
    Comment,
    Account,
    Link,
    Message,
    Subreddit,
    Award,
}

impl<'de> Deserialize<'de> for RedditType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        RedditType::from_str(s.as_str()).map_err(de::Error::custom)
    }
}

impl RedditType {
    pub fn get_id(&self) -> String {
        return match self {
            RedditType::Comment => "t1".to_string(),
            RedditType::Account => "t2".to_string(),
            RedditType::Link => "t3".to_string(),
            RedditType::Message => "t4".to_string(),
            RedditType::Subreddit => "t5".to_string(),
            RedditType::Award => "t6".to_string(),
        };
    }
}

impl FromStr for RedditType {
    type Err = APIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t1" => Ok(RedditType::Comment),
            "t2" => Ok(RedditType::Account),
            "t3" => Ok(RedditType::Link),
            "t4" => Ok(RedditType::Message),
            "t5" => Ok(RedditType::Subreddit),
            "t6" => Ok(RedditType::Message),
            _ => Err(APIError::Custom("Invalid RedditType".to_string())),
        }
    }
}

pub struct FullName {
    pub reddit_type: RedditType,
    pub id: String,
}

impl<'de> Deserialize<'de> for FullName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FullName::from_str(s.as_str()).map_err(de::Error::custom)
    }
}

impl FromStr for FullName {
    type Err = APIError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split("_").collect::<Vec<&str>>();
        if split.len() == 1 {
            // Yes, it is always a good time to make a monty python joke.
            return Err(APIError::Custom("Then shalt thou count to two, no more, no less. Two shall be the number thou shalt count, and the number of the counting shall be two.".to_string()));
        }
        return Ok(FullName {
            reddit_type: RedditType::from_str(split.get(0).unwrap()).unwrap(),
            id: split.get(1).unwrap().to_string(),
        });
    }
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.reddit_type.get_id(), self.id)
    }
}
