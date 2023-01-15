use std::fmt::{Debug, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::{CodeAuthenticator, TokenResponseData, AUTH_CONTENT_TYPE};
use crate::{Authenticator, Authorized, utils};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Body, Client};

use crate::error::http_error::IntoResult;
use crate::error::internal_error::InternalError;
use crate::error::Error;

#[derive(Clone)]
pub struct TokenAuthenticator {
    /// Token
    pub token: Option<String>,
    /// When does it expire
    pub expiration_time: Option<u128>,
    /// Refresh token
    pub refresh_token: String,
    /// Client ID
    client_id: String,
    /// Client Secret
    client_secret: String,
}

impl Debug for TokenAuthenticator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[CodeAuthenticator] Token Defined: {} Expires At {}",
            self.token.is_some(),
            self.expiration_time.unwrap_or(0)
        )
    }
}

impl TokenAuthenticator {
    /// Creates a new Authenticator by Refresh Token Authorization
    ///
    /// Note: The "client_secret" for non-confidential clients (Installed APPs) is an empty string.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<S: Into<String>>(
        client_id: S,
        client_secret: S,
        refresh_token: S,
    ) -> TokenAuthenticator {
        TokenAuthenticator {
            token: None,
            expiration_time: None,
            refresh_token: refresh_token.into(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }
}

impl Authenticator for TokenAuthenticator {
    /// Logs in
    async fn login(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error> {
        let url = "https://www.reddit.com/api/v1/access_token";
        let body = format!(
            "grant_type=refresh_token&refresh_token={}",
            &self.refresh_token
        );
        let mut header = HeaderMap::new();

        header.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&*format!(
                "Basic {}",
                utils::basic_header(&self.client_id, &self.client_secret)
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

        let token: TokenResponseData = response.json().await?;
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
        let body = format!(
            "token={}&token_type_hint=refresh_token",
            &self.refresh_token.to_owned()
        );

        let mut header = HeaderMap::new();
        header.insert(USER_AGENT, HeaderValue::from_str(user_agent).unwrap());
        header.insert(CONTENT_TYPE, AUTH_CONTENT_TYPE.clone());
        let response = client
            .post(url)
            .body(Body::from(body))
            .headers(header)
            .send()
            .await?;
        response.status().into_result()?;
        self.token = None;
        self.expiration_time = None;
        self.refresh_token = "".to_string();
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
        if !self.refresh_token.is_empty() {
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
        Some(self.refresh_token.to_owned())
    }
}

impl Authorized for TokenAuthenticator {}

impl TryInto<TokenAuthenticator> for CodeAuthenticator {
    type Error = Error;

    fn try_into(self) -> Result<TokenAuthenticator, Self::Error> {
        if self.refresh_token.is_none() {
            return Err(Error::Other("No Refresh Token Provided".to_owned()));
        }
        Ok(TokenAuthenticator {
            token: self.token,
            expiration_time: self.expiration_time,
            refresh_token: self.refresh_token.unwrap(),
            client_id: self.client_id,
            client_secret: self.client_secret,
        })
    }
}
