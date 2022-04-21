pub mod auth;
pub mod comments;
pub mod error;
pub mod message;
pub mod responses;
pub mod submission;
pub mod subreddit;
pub mod user;
pub mod utils;
use log::trace;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::Arc;

use reqwest::header::HeaderMap;
use reqwest::{Body, Client as ReqwestClient, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use serde::{de, Deserialize, Deserializer};

use crate::auth::{Authenticator, PasswordAuthenticator};
use crate::error::http_error::IntoResult;
use crate::error::internal_error::InternalError;
use crate::error::Error;
use crate::message::Inbox;
use crate::subreddit::response::Subreddits;
use crate::subreddit::Subreddit;
use crate::user::me::Me;
use crate::user::response::{MeResponse, Users};
use crate::user::User;
use crate::utils::options::FeedOption;
use tokio::sync::{RwLock, RwLockReadGuard};

/// This is who you are. This is your identity and you access point to the Reddit API
#[derive(Clone)]
pub struct Client<A: Authenticator> {
    auth: Arc<RwLock<A>>,
    client: ReqwestClient,
    user_agent: String,
    pub oauth: bool,
}

impl<A: Authenticator> Client<A> {
    /// Logs into Reddit and Returns a Me
    pub async fn login(auth: Arc<RwLock<A>>, user_agent: String) -> Result<Client<A>, Error> {
        let client = ClientBuilder::new()
            .user_agent(user_agent.clone())
            .build()?;
        let arc = auth.clone();
        let mut guard = arc.write().await;
        let b = guard.oauth();
        let _x = guard.login(&client, &user_agent).await;
        Ok(Client {
            auth,
            client,
            user_agent,
            oauth: b,
        })
    }
    /// Gets the authenticator. Internal use
    pub async fn get_authenticator(&self) -> RwLockReadGuard<'_, A> {
        self.auth.read().await
    }
    /// Creates a subreddit object. However, this will not tell you if the user exists.
    pub fn subreddit<T: Into<String>>(&self, name: T) -> Subreddit<A> {
        Subreddit {
            me: self,
            name: name.into(),
        }
    }

    /// Creates a user object. However, this will not tell you if the user exists.
    pub fn user<T: Into<String>>(&self, name: T) -> User<A> {
        User {
            me: self,
            name: name.into(),
        }
    }
    /// Makes a get request with Reqwest response
    pub async fn get(&self, url: &str, oauth: bool) -> Result<Response, Error> {
        let mut guard = self.get_authenticator().await;
        if guard.needs_token_refresh() {
            trace!("Token Expired. Refreshing");
            drop(guard);
            self.re_login();
            guard = self.get_authenticator().await;
        }
        let string = self.build_url(url, oauth, guard.oauth());
        let mut headers = HeaderMap::new();
        guard.headers(&mut headers);
        drop(guard);
        self.client
            .get(string)
            .headers(headers)
            .send()
            .await
            .map_err(|error| Error::InternalError(InternalError::ReqwestError(error)))
    }
    /// Makes a post request with Reqwest response
    pub async fn post(&self, url: &str, oauth: bool, body: Body) -> Result<Response, Error> {
        let mut guard = self.get_authenticator().await;
        if guard.needs_token_refresh() {
            trace!("Token Expired. Refreshing");
            drop(guard);
            self.re_login();
            guard = self.get_authenticator().await;
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
            .map_err(Error::from)
    }
    /// Makes a get request with JSON response
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        url: &str,
        oauth: bool,
    ) -> crate::error::Result<T> {
        let response = self.get(url, oauth).await?;
        response.status().into_result()?;
        let value = response.text().await?;
        trace!("{}", &value);
        let x: T = serde_json::from_str(value.as_str())?;
        Ok(x)
    }
    /// Makes a post request with JSON response
    pub async fn post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        oauth: bool,
        body: Body,
    ) -> crate::error::Result<T> {
        let response = self.post(url, oauth, body).await?;
        response.status().into_result()?;

        let value = response.text().await?;
        trace!("{}", &value);
        let x: T = serde_json::from_str(value.as_str())?;
        Ok(x)
    }
    /// Builds a URL
    pub fn build_url(&self, dest: &str, oauth_required: bool, oauth_supported: bool) -> String {
        let stem = if oauth_required || oauth_supported {
            // All endpoints support OAuth, but some do not support the regular endpoint. If we are
            // required to use it or support it, we will use it.
            assert!(
                oauth_supported,
                "OAuth is required to use this endpoint, but your authenticator does not \
                     support it."
            );
            "https://oauth.reddit.com"
        } else {
            "https://api.reddit.com"
        };
        format!("{stem}{dest}")
    }

    /// Searches Reddit for subreddits
    pub async fn search_subreddits(
        &self,
        name: String,
        limit: Option<u64>,
        feed: Option<FeedOption>,
    ) -> crate::error::Result<Subreddits> {
        let mut url = format!("/subreddits/search?q={name}");
        if let Some(options) = feed {
            url.push_str(options.url().as_str());
        }
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={limit}"));
        }
        self.get_json::<Subreddits>(&url, false).await
    }

    /// Searches Reddit for Users
    pub async fn search_users(
        &self,
        name: String,
        limit: Option<u64>,
        feed: Option<FeedOption>,
    ) -> crate::error::Result<Users> {
        let mut url = format!("/users/search?q={name}");
        if let Some(options) = feed {
            url.push_str(options.url().as_str());
        }
        if let Some(limit) = limit {
            url.push_str(&format!("&limit={limit}"));
        }
        self.get_json::<Users>(&url, false).await
    }
    async fn re_login(&self) {
        let mut guard = self.auth.write().await;
        guard.login(&self.client, &self.user_agent).await;
    }
}
impl Client<PasswordAuthenticator> {
    /// Inbox
    pub fn inbox(&self) -> Inbox {
        Inbox { me: self }
    }

    /// Get Me
    #[allow(clippy::needless_lifetimes)]
    pub async fn me<'a>(&'a self) -> Result<Me<'a>, Error> {
        let me: MeResponse = self.get_json("/api/v1/me", true).await?;
        Ok(Me { client: self, me })
    }
}
pub struct FullName {
    pub reddit_type: String,
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
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split('_').collect::<Vec<&str>>();
        if split.len() == 1 {
            // Yes, it is always a good time to make a monty python joke.
            return Err(Error::from("Then shalt thou count to two, no more, no less. Two shall be the number thou shalt count, and the number of the counting shall be two."));
        }
        return Ok(FullName {
            reddit_type: split.get(0).unwrap().to_string(),
            id: split.get(1).unwrap().to_string(),
        });
    }
}

impl Display for FullName {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.reddit_type, self.id)
    }
}
