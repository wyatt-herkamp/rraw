use std::fmt::{Debug, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Body, Client};

use crate::error::http_error::IntoResult;
use crate::error::internal_error::InternalError;
use crate::error::Error;
pub use serde::Deserialize;

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

#[derive(Clone)]
pub struct PasswordAuthenticator {
    /// Token
    pub token: Option<String>,
    /// When does it expire
    pub expiration_time: Option<u128>,
    /// Client ID
    client_id: String,
    /// Client Secret
    client_secret: String,
    /// Username
    username: String,
    /// Password
    password: String,
}

impl Debug for PasswordAuthenticator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[PasswordAuthenticator] Token Defined: {} Expires At {}",
            self.token.is_some(),
            self.expiration_time.unwrap_or(0)
        )
    }
}

impl PasswordAuthenticator {
    /// Creates a new Authenticator
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: Into<String>>(
        client_id: S,
        client_secret: S,
        username: S,
        password: S,
    ) -> PasswordAuthenticator {
        PasswordAuthenticator {
            token: None,
            expiration_time: None,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}

#[async_trait]
impl Authenticator for PasswordAuthenticator {
    /// Logs in
    async fn login(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error> {
        let url = "https://www.reddit.com/api/v1/access_token";
        let body = format!(
            "grant_type=password&username={}&password={}",
            &self.username, &self.password
        );
        let mut header = HeaderMap::new();
        header.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&*format!(
                "Basic {}",
                base64::encode(format!(
                    "{}:{}",
                    self.client_id.to_owned(),
                    self.client_secret.to_owned()
                ))
            ))
                .unwrap(),
        );
        header.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        let response = client
            .post(url)
            .body(Body::from(body))
            .headers(header)
            .send()
            .await
            .map_err(InternalError::from)?;
        response.status().into_result()?;

        let token = response.text().await?;
        let token: TokenResponseData = serde_json::from_str(token.as_str())?;
        self.token = Some(token.access_token);
        let x = token.expires_in * 1000;
        let x1 = (x as u128)
            + SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        self.expiration_time = Some(x1);
        return Ok(true);
    }
    /// Logs out
    async fn logout(&mut self, client: &Client, user_agent: &str) -> Result<(), Error> {
        let url = "https://www.reddit.com/api/v1/revoke_token";
        let body = format!("token={}", &self.token.to_owned().unwrap());

        let mut header = HeaderMap::new();
        header.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        let response = client
            .post(url)
            .body(Body::from(body))
            .headers(header)
            .send()
            .await?;
        response.status().into_result()?;
        self.token = None;
        self.expiration_time = None;
        Ok(())
    }
    /// Returns true if successful
    async fn token_refresh(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error> {
        self.login(client, user_agent).await
    }
    /// headers
    fn headers(&self, headers: &mut HeaderMap) {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&*format!("Bearer {}", self.token.to_owned().unwrap())).unwrap(),
        );
    }
    /// True
    fn oauth(&self) -> bool {
        true
    }
    /// Validates Time
    fn needs_token_refresh(&self) -> bool {
        let i = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if self.expiration_time.is_none() {
            true
        } else {
            i >= self.expiration_time.unwrap()
        }
    }
    /// Always None
    fn get_refresh_token(&self) -> Option<String> {
        Option::None
    }
}

#[derive(Clone)]
pub struct CodeAuthenticator {
    /// Token
    pub token: Option<String>,
    /// When does it expire
    pub expiration_time: Option<u128>,
    /// Refresh token
    pub refresh_token: Option<String>,
    /// Client ID
    client_id: String,
    /// Client Secret
    client_secret :String,
    /// Authorization code
    authorization_code: String,
    /// Redirect URI 
    redirect_uri: String,
}

impl Debug for CodeAuthenticator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[CodeAuthenticator] Token Defined: {} Expires At {} And Refrsh Token Defined: {}",
            self.token.is_some(),
            self.expiration_time.unwrap_or(0),
            self.refresh_token.is_some()
        )
    }
}

impl CodeAuthenticator {
    /// Creates a new Authenticator by Code Flow
    /// The "client_secret" for non-confidential clients (Installed APPs) is an empty string.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: Into<String>>(
        client_id: S,
        client_secret: S,
        authorization_code: S,
        redirect_uri: S,
    ) -> CodeAuthenticator {
        CodeAuthenticator {
            token: None,
            expiration_time: None,
            refresh_token: None,
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            authorization_code: authorization_code.into(),
            redirect_uri: redirect_uri.into(),
        }
    }
}

#[async_trait]
impl Authenticator for CodeAuthenticator {
    /// Logs in
    async fn login(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error> {
        let url = "https://www.reddit.com/api/v1/access_token";
        let body = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}",
            &self.authorization_code.trim_end_matches("#_"), &self.redirect_uri
        );
        let mut header = HeaderMap::new();
        header.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&*format!(
                "Basic {}",
                base64::encode(format!(
                    "{}:{}",
                    self.client_id.to_owned(),
                    self.client_secret.to_owned()
                ))
            ))
            .unwrap(),
        );
        header.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        let response = client
            .post(url)
            .body(Body::from(body))
            .headers(header)
            .send()
            .await
            .map_err(InternalError::from)?;
        response.status().into_result()?;

        let token = response.text().await?;
        let token: TokenResponseData = serde_json::from_str(token.as_str())?;
        self.token = Some(token.access_token);
        let x = token.expires_in * 1000;
        let x1 = (x as u128)
            + SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
        self.expiration_time = Some(x1);
        if !token.refresh_token.is_empty() {
            self.refresh_token = Some(token.refresh_token);
        }        
        return Ok(true);
    }
    /// Logs out
    async fn logout(&mut self, client: &Client, user_agent: &str) -> Result<(), Error> {
        let url = "https://www.reddit.com/api/v1/revoke_token";
        let body: String;
        if self.refresh_token.is_none() {
            body = format!("token={}&token_type_hint=access_token", &self.token.to_owned().unwrap());
        } else {
            body = format!("token={}&token_type_hint=refresh_token", &self.refresh_token.to_owned().unwrap());
        }

        let mut header = HeaderMap::new();
        header.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        let response = client
            .post(url)
            .body(Body::from(body))
            .headers(header)
            .send()
            .await?;
        response.status().into_result()?;
        self.token = None;
        self.expiration_time = None;
        self.refresh_token = None;
        Ok(())
    }
    /// Returns true if successful
    async fn token_refresh(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error> {
        let url = "https://www.reddit.com/api/v1/access_token";
        let body = format!(
            "grant_type=refresh_token&refresh_token={}",
            &self.refresh_token.to_owned().unwrap()
        );
        let mut header = HeaderMap::new();
        header.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&*format!(
                "Basic {}",
                base64::encode(format!(
                    "{}:",
                    self.client_id.to_owned()
                ))
            ))
            .unwrap(),
        );
        header.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        header.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        let response = client
            .post(url)
            .body(Body::from(body))
            .headers(header)
            .send()
            .await
            .map_err(InternalError::from)?;
        response.status().into_result()?;

        let token = response.text().await?;
        let token: TokenResponseData = serde_json::from_str(token.as_str())?;
        self.token = Some(token.access_token);
        let x = token.expires_in * 1000;
        let x1 = (x as u128)
            + SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
        self.expiration_time = Some(x1);
        return Ok(true);
    }
    /// headers
    fn headers(&self, headers: &mut HeaderMap) {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&*format!("Bearer {}", self.token.to_owned().unwrap())).unwrap(),
        );
    }
    /// True
    fn oauth(&self) -> bool {
        true
    }
    /// Validates Time
    fn needs_token_refresh(&self) -> bool {
        let i = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if self.refresh_token.is_some() {
            if self.expiration_time.is_none() {
                true
            } else {
                i >= self.expiration_time.unwrap()
            }
        } else {
            false
        }       
    }
    fn get_refresh_token(&self) -> Option<String> {
        self.refresh_token.to_owned()            
    }
}