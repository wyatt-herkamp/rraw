use std::fmt::{Debug, Formatter};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::{TokenResponseData, AUTH_CONTENT_TYPE};
use crate::{Authenticator, Authorized, utils};
use log::warn;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use reqwest::{Body, Client};

use crate::error::http_error::IntoResult;
use crate::error::internal_error::InternalError;
use crate::error::Error;

#[derive(Clone)]
pub struct CodeAuthenticator {
    /// Token
    pub token: Option<String>,
    /// When does it expire
    pub expiration_time: Option<u128>,
    /// Refresh token
    pub refresh_token: Option<String>,
    /// Client ID
    pub(crate) client_id: String,
    /// Client Secret
    pub(crate) client_secret: String,
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
    ///
    /// Note: The "client_secret" for non-confidential clients (Installed APPs) is an empty string.
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
    /// This method does not check the values of the parameters.
    ///
    /// Information of the data can be found [here](https://github.com/reddit-archive/reddit/wiki/OAuth2).
    pub fn generate_authorization_url(
        client_id: impl AsRef<str>,
        redirect_uri: impl AsRef<str>,
        state: impl AsRef<str>,
        duration: impl AsRef<str>,
        scope: Vec<&str>,
    ) -> String {
        format!(
            "https://www.reddit.com/api/v1/authorize?client_id={}&response_type=code&state={}&redirect_uri={}&duration={}&scope={}",
            client_id.as_ref(),
            state.as_ref(),
            redirect_uri.as_ref(),
            duration.as_ref(),
            scope.join(",")
        )
    }
}

impl Authenticator for CodeAuthenticator {
    /// Logs in
    async fn login(&mut self, client: &Client, user_agent: &str) -> Result<bool, Error> {
        let url = "https://www.reddit.com/api/v1/access_token";
        let body = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}",
            &self.authorization_code.trim_end_matches("#_"),
            &self.redirect_uri
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
        if !token.refresh_token.is_empty() {
            self.refresh_token = Some(token.refresh_token);
        }
        return Ok(true);
    }
    /// Logs out
    async fn logout(&mut self, client: &Client, user_agent: &str) -> Result<(), Error> {
        let url = "https://www.reddit.com/api/v1/revoke_token";
        let body = if let Some(refresh_token) = &self.refresh_token {
            format!("token={}&token_type_hint=refresh_token", refresh_token)
        } else if let Some(token) = self.token.as_ref() {
            format!("token={}&token_type_hint=access_token", token)
        } else {
            // No token to revoke
            return Ok(());
        };

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
    // headers
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

impl Authorized for CodeAuthenticator {}
