use crate::responses::{GenericListing, GenericResponse};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub use serde::Deserialize;

use serde_json::Value;

///About Data for the User
#[derive(Deserialize, Clone)]
pub struct MeResponse {
    #[serde(flatten)]
    pub about: AboutUser,
    /// If you know what this data is. Please Tell me.
    pub features: Value,
}
impl Debug for MeResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Me Response] User: {}", self.about.name)
    }
}
#[derive(Deserialize, Clone)]
pub struct PersonalInformation {
    pub pref_no_profanity: bool,
    pub has_external_account: bool,
    pub pref_geopopular: String,
    pub pref_show_trending: bool,
    pub pref_show_presence: bool,
    /// IDK
    pub gold_expiration: Option<i64>,
    /// I am guessing premium?
    pub has_gold_subscription: bool,
    /// users gold
    pub coins: i64,
    /// has_paypal_subscription - Why is this public?
    pub has_paypal_subscription: bool,
    /// has_subscribed_to_premium
    pub has_subscribed_to_premium: bool,
}
impl Debug for PersonalInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Personal Information]")
    }
}
#[derive(Deserialize, Clone)]
pub struct AboutUser {
    pub is_employee: bool,
    pub is_friend: bool,
    //TODO expand upon later
    pub subreddit: Value,
    pub snoovatar_size: Option<Vec<i64>>,
    pub awardee_karma: i64,
    pub id: String,
    pub verified: bool,
    pub is_gold: bool,
    pub is_mod: bool,
    pub awarder_karma: i64,
    pub has_verified_email: bool,
    pub icon_img: String,
    pub hide_from_robots: bool,
    pub link_karma: i64,
    pub is_blocked: bool,
    pub total_karma: i64,
    pub pref_show_snoovatar: bool,
    pub name: String,
    pub created: i64,
    pub created_utc: i64,
    pub snoovatar_img: String,
    pub comment_karma: i64,
    pub accept_followers: bool,
    pub has_subscribed: bool,
    #[serde(flatten)]
    pub personal_details: Option<PersonalInformation>,
}
impl Debug for AboutUser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[User]. name: {}. Contains Personal Information {}",
            self.name,
            self.personal_details.is_some()
        )
    }
}

/// About with a GenericResponse Wrap
pub type UserResponse = GenericResponse<AboutUser>;
/// A listing of user abouts
pub type Users = GenericListing<AboutUser>;
