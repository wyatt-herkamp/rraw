use crate::responses::{GenericResponse, GenericListing};
pub use serde::Deserialize;
use serde_json::Value;

///About Data for the User
#[derive(Deserialize, Debug)]
pub struct AboutUser {
    /// The Reddit generated avatar image
    pub snoovatar_img: Option<String>,
    /// classic icon img
    pub icon_img: Option<String>,
    /// Reddit Username
    pub name: String,
    /// Is the user an employee? I think
    pub is_employee: bool,
    /// IDK
    pub has_visited_new_profile: Option<bool>,
    /// IDK
    pub is_friend: bool,
    /// IDK
    pub pref_no_profanity: Option<bool>,
    /// IDK
    pub has_external_account: Option<bool>,
    /// I am guessing location
    pub pref_geopopular: Option<bool>,
    /// IDK
    pub pref_show_trending: Option<bool>,
    /// IDK
    pub pref_show_presence: Option<bool>,
    /// IDK
    pub gold_expiration: Option<bool>,
    /// I am guessing premium?
    pub has_gold_subscription: Option<bool>,
    /// IDK
    pub is_sponsor: Option<bool>,
    /// Number of friends. It is always 0. :(
    pub  num_friends: Option<u64>,
    /// You can edit names?
    pub can_edit_name: Option<bool>,
    /// Verified
    pub verified: Option<bool>,
    /// new_modmail_exists
    pub new_modmail_exists: Option<bool>,
    /// pref_autoplay
    pub pref_autoplay: Option<bool>,
    /// users gold
    pub coins: Option<u64>,
    /// has_paypal_subscription - Why is this public?
    pub has_paypal_subscription: Option<bool>,
    /// has_subscribed_to_premium
    pub has_subscribed_to_premium: Option<bool>,
    /// id
    pub id: String,
    /// has_stripe_subscription
    pub has_stripe_subscription: Option<bool>,
    /// can_create_subreddit
    pub can_create_subreddit: Option<bool>,
    /// Can they visit the fun subs :)
    pub over_18: Option<bool>,
    /// is_gold
    pub is_gold: Option<bool>,
    /// Are they a buzzkill
    pub is_mod: Option<bool>,
    /// awarder_karma
    pub awarder_karma: u64,
    /// suspension_expiration_utci
    pub suspension_expiration_utc: Option<i64>,
    /// has_verified_email
    pub has_verified_email: Option<bool>,
    /// is suspended
    pub is_suspended: Option<bool>,
    ///pref_video_autoplay
    pub pref_video_autoplay: Option<bool>,
    /// in_chat
    pub in_chat: Option<bool>,
    /// has_android_subscription
    pub has_android_subscription: Option<bool>,
    /// in_redesign_beta
    pub  in_redesign_beta: Option<bool>,
    ///has_mod_mail
    pub has_mod_mail: Option<bool>,
    ///pref_nightmode
    pub pref_nightmode: Option<bool>,
    /// awardee_karma
    pub awardee_karma: u64,
    /// hide_from_robots
    pub hide_from_robots: Option<bool>,
    /// password_set
    pub password_set: Option<bool>,
    /// modhash
    pub modhash: Option<bool>,
    /// link_karma
    pub link_karma: u64,
    ///force_password_reset
    pub force_password_reset: Option<bool>,
    /// total_karma
    pub total_karma: u64,
    ///inbox_count
    pub inbox_count: Option<bool>,
    /// pref_top_karma_subreddits
    pub pref_top_karma_subreddits: Option<bool>,
    /// has_mail
    pub has_mail: Option<bool>,
    /// pref_show_snoovatar
    pub pref_show_snoovatar: Option<bool>,
    /// pref_clickgadget
    pub pref_clickgadget: Option<bool>,
    /// created
    pub created: f64,
    /// gold_creddits
    pub gold_creddits: Option<u64>,
    /// created_utc
    pub created_utc: f64,
    /// has_ios_subscription
    pub has_ios_subscription: Option<bool>,
    /// pref_show_twitter
    pub pref_show_twitter: Option<bool>,
    /// in_beta
    pub in_beta: Option<bool>,
    /// comment_karma
    pub comment_karma: u64,
    /// has_subscribed
    pub has_subscribed: Option<bool>,

}


/// About with a GenericResponse Wrap
pub type About = GenericResponse<AboutUser>;
/// A listing of user abouts
pub type Users = GenericListing<AboutUser>;