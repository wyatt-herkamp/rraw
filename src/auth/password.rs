use std::fmt::{Debug, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use async_trait::async_trait;
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Body, Client};
use crate::auth::{AUTH_CONTENT_TYPE, TokenResponseData};
use crate::{Authenticator, Authorized};

use crate::error::http_error::IntoResult;
use crate::error::internal_error::InternalError;
use crate::error::Error;

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
            AUTH_CONTENT_TYPE.clone(),
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
        if let Some(token) = self.token.as_ref() {
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
        } else {
            warn!("No token found");
        }
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

impl Authorized for PasswordAuthenticator {}
