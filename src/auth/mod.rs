use std::fmt::{Debug, Formatter};

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client};
use serde::Deserialize;

use crate::error::Error;

mod token;
mod code;
mod password;

pub use token::TokenAuthenticator;
pub use code::CodeAuthenticator;
pub use password::PasswordAuthenticator;

pub static AUTH_CONTENT_TYPE: HeaderValue = HeaderValue::from_static("application/x-www-form-urlencoded");

#[derive(Deserialize, Debug)]
pub struct TokenResponseData {
    pub access_token: String,
    pub expires_in: u64,
    pub scope: String,
    pub token_type: String,
    #[serde(default = "default_response")]
    pub refresh_token: String,
}

fn default_response() -> String {
    "".to_string()
}

#[async_trait]
pub trait Authenticator: Clone + Send + Sync + Debug {
    /// Logins to the Reddit API
    /// true if successful
    async fn login(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error>;
    /// Releases the token back to Reddit
    async fn logout(&mut self, client: &Client, user_agent: &str) -> Result<(), Error>;
    /// true if successful
    async fn token_refresh(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error>;
    /// Header Values required for auth
    fn headers(&self, headers: &mut HeaderMap);
    /// Supports OAuth
    fn oauth(&self) -> bool;
    /// Does the Token need refresh
    fn needs_token_refresh(&self) -> bool;
    /// Returns refresh token
    fn get_refresh_token(&self) -> Option<String>;
}

pub trait Authorized: Authenticator {}

/// AnonymousAuthenticator
#[derive(Clone, Default)]
pub struct AnonymousAuthenticator;

impl Debug for AnonymousAuthenticator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[AnonymousAuthenticator]")
    }
}

#[async_trait]
impl Authenticator for AnonymousAuthenticator {
    /// Returns true because it is anonymous
    async fn login(&mut self, _client: &Client, _user_agent: &str) -> Result<bool, Error> {
        Ok(true)
    }
    /// Does nothing
    async fn logout(&mut self, _client: &Client, _user_agent: &str) -> Result<(), Error> {
        Ok(())
    }
    /// Returns true because it is anonymous
    async fn token_refresh(&mut self, _client: &Client, _user_agent: &str) -> Result<bool, Error> {
        Ok(true)
    }
    /// Does Nothing
    fn headers(&self, _headers: &mut HeaderMap) {}
    /// False because not logged in
    fn oauth(&self) -> bool {
        false
    }
    /// Always false
    fn needs_token_refresh(&self) -> bool {
        false
    }
    /// Always None
    fn get_refresh_token(&self) -> Option<String> {
        Option::None
    }
}

impl AnonymousAuthenticator {
    /// Creates a new Authenticator
    pub fn new() -> AnonymousAuthenticator {
        AnonymousAuthenticator
    }
}
