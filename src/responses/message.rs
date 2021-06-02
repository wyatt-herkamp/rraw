use serde::{Deserialize};
use serde_json::Value;
#[derive(Deserialize, Debug)]
pub struct Message{
    pub associated_awarding_id: Option<Value>,
    pub author: Option<String>,
    pub author_fullname: Option<Value>,
    pub body: Option<String>,
    pub body_html: Option<String>,
    pub context: Option<String>,
    pub created: Option<f64>,
    pub created_utc: Option<f64>,
    pub dest: Option<String>,
    pub distinguished: Option<String>,
    pub first_message: Option<Value>,
    pub first_message_name: Option<Value>,
    pub id: Option<String>,
    pub likes: Option<Value>,
    pub name: Option<String>,
    pub new: Option<bool>,
    pub num_comments: Option<Value>,
    pub parent_id: Option<Value>,
    pub replies: Option<String>,
    pub score: Option<i64>,
    pub subject: Option<String>,
    pub subreddit: Option<Value>,
    pub subreddit_name_prefixed: Option<Value>,
    #[serde(rename(deserialize = "type"))]
    pub type_: Option<String>,
    pub was_comment: Option<bool>,


}