use crate::auth::Auth;
use crate::subreddit::Subreddit;
use crate::user::User;
use crate::utils::error::APIError;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use reqwest::{Body, Client, Response};
use serde::de::DeserializeOwned;

use crate::responses::subreddit::Subreddits;
use crate::responses::user::Users;
use crate::responses::RedditListing;
use crate::utils::options::FeedOption;
use serde::Serialize;
use std::sync::{Arc, Mutex, MutexGuard};

/// This is who you are. This is your identity and you access point to the Reddit API

pub struct Me {
    auth: Arc<Mutex<Box<dyn Auth>>>,
    client: Client,
    user_agent: String,
}

impl Me {
    /// Logs into Reddit and Returns a Me
    pub async fn login(
        auth: Arc<Mutex<Box<dyn Auth>>>,
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
    pub fn get_authenticator(&self) -> MutexGuard<Box<dyn Auth + 'static>> {
        self.auth.lock().unwrap()
    }
    /// Creates a subreddit object. However, this will not tell you if the user exists.
    pub fn subreddit(&self, name: String) -> Subreddit {
        Subreddit { me: self, name }
    }
    /// Creates a user object. However, this will not tell you if the user exists.
    pub fn user(&self, name: String) -> User {
        User { me: self, name }
    }
    /// Makes a get request with Reqwest response
    pub async fn get(&self, url: &str, oauth: bool) -> Result<Response, reqwest::Error> {
        let string = self.build_url(url, oauth);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&*self.user_agent).unwrap(),
        );
        self.get_authenticator().headers(&mut headers);
        self.client.get(string).headers(headers).send().await
    }
    /// Makes a post request with Reqwest response
    pub async fn post(
        &self,
        url: &str,
        oauth: bool,
        body: Body,
    ) -> Result<Response, reqwest::Error> {
        let string = self.build_url(url, oauth);
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&*self.user_agent).unwrap(),
        );
        self.get_authenticator().headers(&mut headers);
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
