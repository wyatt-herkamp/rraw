use reqwest::header::{
    HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
};
use reqwest::{Body, Client};


use crate::responses::other::TokenResponseData;
use crate::utils::error::APIError;
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

#[async_trait]
pub trait Auth {
    async fn login(&mut self, client: &Client, user_agent: &str) -> Result<bool, APIError>;
    async fn refresh_token(&mut self, client: &Client, user_agent: &str) -> Result<bool, APIError> {
        self.login(client, user_agent).await
    }
    async fn logout(&mut self, client: &Client, user_agent: &str) -> Result<(), APIError>;

    fn headers(&self, headers: &mut HeaderMap);
    fn oauth(&self) -> bool;

    fn needs_token_refresh(&self) -> bool;
}

pub struct AnonymousAuthenticator;

#[async_trait]
impl Auth for AnonymousAuthenticator {
    async fn login(&mut self, _client: &Client, _user_agent: &str) -> Result<bool, APIError> {
        Ok(true)
    }

    async fn logout(&mut self, _client: &Client, _user_agent: &str) -> Result<(), APIError> {
        Ok(())
    }

    fn headers(&self, _headers: &mut HeaderMap) {}

    fn oauth(&self) -> bool {
        false
    }

    fn needs_token_refresh(&self) -> bool {
        false
    }
}

impl AnonymousAuthenticator {
    pub fn new() -> Arc<Mutex<Box<dyn Auth>>> {
        Arc::new(Mutex::new(Box::new(AnonymousAuthenticator {})))
    }
}

pub struct PasswordAuthenticator {
    pub token: Option<String>,
    pub expiration_time: Option<u128>,
    client_id: String,
    client_secret: String,
    username: String,
    password: String,
}

impl PasswordAuthenticator {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        username: &str,
        password: &str,
    ) -> Arc<Mutex<Box<dyn Auth + Send>>> {
        Arc::new(Mutex::new(Box::new(PasswordAuthenticator {
            token: None,
            expiration_time: None,
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
        })))
    }
}

#[async_trait]
impl Auth for PasswordAuthenticator {
    async fn login(&mut self, client: &Client, user_agent: &str) -> Result<bool, APIError> {
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
        let result2 = client
            .post(url)
            .body(Body::from(body))
            .headers(header)
            .send()
            .await;
        if let Ok(response) = result2 {
            let code = response.status();
            if !code.is_success() {
                return Err(APIError::HTTPError(code));
            }
            let value = response.json::<TokenResponseData>().await;
            if let Ok(token) = value {
                self.token = Some(token.access_token);
                let x = token.expires_in * 1000;
                let x1 = (x as u128)
                    + SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();
                self.expiration_time = Some(x1);
                return Ok(true);
            } else if let Err(response) = value {
                return Err(APIError::from(response));
            }
        } else if let Err(response) = result2 {
            return Err(APIError::from(response));
        }
        return Err(APIError::ExhaustedListing);
    }

    async fn logout(&mut self, _client: &Client, _user_agent: &str) -> Result<(), APIError> {
        todo!()
    }

    fn headers(&self, headers: &mut HeaderMap) {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&*format!("Bearer {}", self.token.to_owned().unwrap())).unwrap(),
        );
    }

    fn oauth(&self) -> bool {
        true
    }

    fn needs_token_refresh(&self) -> bool {
        let i = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        return if self.expiration_time.is_none() {
            true
        } else {
            i >= self.expiration_time.unwrap()
        };
    }
}
