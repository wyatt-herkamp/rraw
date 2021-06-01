use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::responses::{GenericListing, GenericResponse};

#[derive(Debug, Deserialize)]
pub struct Comment {
    pub link_id: Option<String>,
    pub likes: Option<bool>,
    pub id: Option<String>,
    pub author: Option<String>,
    pub created_utc: Option<f64>,
    pub parent_id: Option<String>,
    pub score: Option<i32>,
    pub author_fullname: Option<String>,
    pub subreddit_id: Option<String>,
    pub body: Option<String>,
    pub link_title: Option<String>,
    pub name: Option<String>,
    pub downs: Option<i32>,
    pub body_html: Option<String>,
    pub distinguished: Option<String>,
    pub stickied: Option<bool>,
    pub ups: Option<i32>,
}

impl<'de> Deserialize<'de> for GenericResponse<Comment> {
    fn deserialize<D>(deserializer: D) -> Result<GenericResponse<Comment>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: serde_json::Value = serde::Deserialize::deserialize(deserializer).unwrap();
        let result = serde_json::from_value(value);
        if let Err(e) = result {
            return Err(D::Error::custom(e.to_string()));
        }
        return Ok(result.unwrap());
    }
}

pub type Comments = GenericResponse<Comment>;
