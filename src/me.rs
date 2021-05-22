use std::sync::{Arc, Mutex, MutexGuard};
use crate::auth::Auth;
use reqwest::{Client, Response, Body};
use crate::utils::error::APIError;
use crate::subreddit::Subreddit;
use reqwest::header::{HeaderValue, USER_AGENT, HeaderMap};
use serde::Deserialize;
use serde::de::DeserializeOwned;
use crate::user::User;

pub struct Me {
    auth: Arc<Mutex<Box<dyn Auth>>>,
    client: Client,
    user_agent: String,
}

impl Me {
    pub async fn login(auth: Arc<Mutex<Box<dyn Auth>>>, user_agent: String) -> Result<Me, APIError> {
        let client = Client::new();
        let x = auth.lock().unwrap().login(&client, &user_agent).await;
        Ok(Me {
            auth,
            client,
            user_agent,
        })
    }
    pub fn get_authenticator(&self) -> MutexGuard<Box<Auth + 'static>> {
        self.auth.lock().unwrap()
    }
    pub fn subreddit(&self, name: String) -> Subreddit {
        Subreddit {
            me: self,
            name,
        }
    }
    pub fn user(&self, name: String) -> User {
        User {
            me: self,
            name,
        }
    }
    pub async fn get(&self, url: &str, oauth: bool) -> Result<Response, reqwest::Error> {
        let string = self.build_url(url, oauth);
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&*self.user_agent).unwrap());
        self.get_authenticator().headers(&mut headers);
        self.client.get(string).headers(headers).send().await
    }
    pub async fn post(&self, url: &str, oauth: bool, body: String) -> Result<Response, reqwest::Error> {
        let string = self.build_url(url, oauth);
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&*self.user_agent).unwrap());
        self.get_authenticator().headers(&mut headers);
        self.client.post(string).headers(headers).body(Body::from(body)).send().await
    }
    pub async fn get_json<T: DeserializeOwned>(&self, url: &str, oauth: bool) -> Result<T, APIError> {
        let x = self.get(url, oauth).await;
        return Me::respond::<T>(x).await;
    }

    pub fn build_url(&self, dest: &str, oauth_required: bool)
                     -> String {
        let stem = if oauth_required {
            "https://oauth.reddit.com"
        } else {
            "https://api.reddit.com"
        };
        format!("{}{}", stem, dest)
    }
    pub async fn respond<T: DeserializeOwned>(result: Result<Response, reqwest::Error>) -> Result<T, APIError> {
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
}
