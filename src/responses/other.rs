pub use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TokenResponseData {
    pub access_token: String,
    pub expires_in: u64,
    pub scope: String,
    pub token_type: String,
}