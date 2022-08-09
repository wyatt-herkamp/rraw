use std::collections::HashMap;
use crate::responses::GenericResponse;
use std::fmt::{Debug, Display, Formatter};

use crate::responses::listing::GenericListing;
pub use serde::Deserialize;
use serde_json::Value;

/// The response from an add friend request
#[derive(Debug, Deserialize)]
pub struct Friend {
    /// Was the friend request a success
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct Moderator {
    pub name: String,
    pub author_flair_text: Option<String>,
    pub author_flair_css_class: Option<String>,
    pub date: u64,
    pub mod_permissions: Vec<String>,
}

pub type Moderators = GenericListing<Moderator>;

#[derive(Debug, Deserialize)]
pub struct Contributor {
    pub name: String,
    pub id: Option<String>,
    pub rel_id: Option<String>,
    pub date: u64,
}

pub type Contributors = GenericListing<Contributor>;

#[derive(Deserialize, Clone)]
pub struct AboutSubreddit {
    pub name: String,
    pub display_name: String,
    pub url: String,
    pub title: Option<String>,
    pub created: f64,
    pub created_utc: f64,
    #[serde(default)]
    pub subscribers: u64,
    #[serde(default)]
    pub over18: bool,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

impl Display for AboutSubreddit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name)
    }
}

impl Debug for AboutSubreddit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Subreddit]. Permalink: {}", self.url)
    }
}

pub type SubredditResponse = GenericResponse<AboutSubreddit>;
pub type Subreddits = GenericListing<AboutSubreddit>;
