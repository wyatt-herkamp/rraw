use crate::responses::listing::GenericListing;
use crate::responses::{FullName, GenericResponse};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Message {
    pub associated_awarding_id: Option<Value>,
    pub author: String,
    pub author_fullname: Option<FullName>,
    pub body: Option<String>,
    pub body_html: Option<String>,
    pub context: Option<String>,
    pub created: f64,
    pub created_utc: f64,
    pub dest: Option<String>,
    pub distinguished: Option<String>,
    pub first_message: Option<Value>,
    pub first_message_name: Option<Value>,
    pub id: String,
    pub likes: Option<bool>,
    pub name: String,
    pub new: Option<bool>,
    pub num_comments: Option<Value>,
    pub parent_id: Option<Value>,
    pub replies: Option<String>,
    pub score: f64,
    pub subject: String,
    pub subreddit: Option<String>,
    pub subreddit_name_prefixed: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub type_: Option<String>,
    #[serde(default)]
    pub was_comment: bool,
}
/// About with a GenericResponse Wrap
pub type MessageResponse = GenericResponse<Message>;
/// A listing of user abouts
pub type MessageListing = GenericListing<Message>;
