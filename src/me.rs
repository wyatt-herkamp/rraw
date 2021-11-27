use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::{Arc};
use log::trace;

use reqwest::{Body, Client, ClientBuilder, Response};
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
use tokio::sync::{Mutex, MutexGuard};

/// This is who you are. This is your identity and you access point to the Reddit API
#[derive(Clone)]
pub struct Me {
    auth: Arc<Mutex<Box<dyn Authenticator + Send>>>,
    client: Client,
    user_agent: String,
    pub oauth: bool,
}

impl Me {
    /// Logs into Reddit and Returns a Me
    pub async fn login(
        auth: Arc<Mutex<Box<dyn Authenticator + Send>>>,
        user_agent: String,
    ) -> Result<Me, APIError> {
        let client = ClientBuilder::new().user_agent(user_agent.clone()).build()?;
        let arc = auth.clone();
        let mut guard = arc.lock().await;
        let b = guard.oauth();
        let _x = guard.login(&client, &user_agent).await;
        Ok(Me {
            auth,
            client,
            user_agent,
            oauth: b,
        })
    }
    /// Gets the authenticator. Internal use
    pub async fn get_authenticator(&self) -> MutexGuard<'_, Box<dyn Authenticator + 'static + Send>> {
        self.auth.lock().await
    }
    /// Creates a subreddit object. However, this will not tell you if the user exists.
    pub fn subreddit<T: Into<String>>(&self, name: T) -> Subreddit {
        Subreddit { me: self, name: name.into() }
    }
    /// Inbox
    pub fn inbox(&self) -> Inbox {
        Inbox { me: self }
    }
    /// Creates a user object. However, this will not tell you if the user exists.
    pub fn user<T: Into<String>>(&self, name: T) -> User {
        User { me: self, name: name.into() }
    }
    /// Makes a get request with Reqwest response
    pub async fn get(&self, url: &str, oauth: bool) -> Result<Response, reqwest::Error> {
        let mut guard = self.get_authenticator().await;
        if guard.needs_token_refresh() {
            guard.login(&self.client, self.user_agent.as_str()).await;
        }
        let string = self.build_url(url, oauth, guard.oauth());
        let mut headers = HeaderMap::new();
        guard.headers(&mut headers);
        drop(guard);
        self.client.get(string).headers(headers).send().await
    }
    /// Makes a post request with Reqwest response
    pub async fn post(
        &self,
        url: &str,
        oauth: bool,
        body: Body,
    ) -> Result<Response, reqwest::Error> {
        let mut guard = self.get_authenticator().await;
        if guard.needs_token_refresh() {
            guard.login(&self.client, self.user_agent.as_str()).await;
        }
        let string = self.build_url(url, oauth, guard.oauth());
        let mut headers = HeaderMap::new();
        guard.headers(&mut headers);
        drop(guard);
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
        let response = self.get(url, oauth).await?;
        if !response.status().is_success() {
            trace!("Bad Response Status {}", response.status().as_u16() );
            return Err(response.status().clone().into());
        }
        let value = response.text().await?;
        trace!("{}",&value);
        let x: T = serde_json::from_str(value.as_str())?;
        return Ok(x);
    }
    /// Makes a post request with JSON response
    pub async fn post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        oauth: bool,
        body: Body,
    ) -> Result<T, APIError> {
        let response = self.post(url, oauth, body).await?;
        if !response.status().is_success() {
            trace!("Bad Response Status {}", response.status().as_u16() );
            return Err(response.status().clone().into());
        }
        let value = response.text().await?;
        trace!("{}",&value);
        let x: T = serde_json::from_str(value.as_str())?;
        return Ok(x);
    }
    /// Builds a URL
    pub fn build_url(&self, dest: &str, oauth_required: bool, oauth_supported: bool) -> String {
        let stem = if oauth_required || oauth_supported {
            // All endpoints support OAuth, but some do not support the regular endpoint. If we are
            // required to use it or support it, we will use it.
            assert!(oauth_supported,
                    "OAuth is required to use this endpoint, but your authenticator does not \
                     support it.");
            "https://oauth.reddit.com"
        } else {
            "https://api.reddit.com"
        };
        format!("{}{}", stem, dest)
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
