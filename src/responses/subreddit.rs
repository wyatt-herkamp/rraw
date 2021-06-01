use crate::responses::{GenericListing, GenericResponse};
use crate::subreddit::Subreddit;
use serde::de::Error;
pub use serde::Deserialize;
use serde::Deserializer;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct AboutSubreddit {
    pub accounts_active: Option<i64>,
    pub accounts_active_is_fuzzed: Option<bool>,
    pub active_user_count: Option<i64>,
    pub advertiser_category: Option<String>,
    pub all_original_content: Option<bool>,
    pub allow_chat_post_creation: Option<bool>,
    pub allow_discovery: Option<bool>,
    pub allow_galleries: Option<bool>,
    pub allow_images: Option<bool>,
    pub allow_polls: Option<bool>,
    pub allow_predictions: Option<bool>,
    pub allow_predictions_tournament: Option<bool>,
    pub allow_videogifs: Option<bool>,
    pub allow_videos: Option<bool>,
    pub banner_background_color: Option<String>,
    pub banner_background_image: Option<String>,
    pub banner_img: Option<String>,
    pub banner_size: Option<Value>,
    pub can_assign_link_flair: Option<bool>,
    pub can_assign_user_flair: Option<bool>,
    pub coins: Option<i64>,
    pub collapse_deleted_comments: Option<bool>,
    pub collections_enabled: Option<bool>,
    pub comment_score_hide_mins: Option<i64>,
    pub community_icon: Option<String>,
    pub community_reviewed: Option<bool>,
    pub created: Option<f64>,
    pub created_utc: Option<f64>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub disable_contributor_requests: Option<bool>,
    pub display_name: Option<String>,
    pub display_name_prefixed: Option<String>,
    pub emojis_custom_size: Option<Value>,
    pub emojis_enabled: Option<bool>,
    pub event_posts_enabled: Option<bool>,
    pub free_form_reports: Option<bool>,
    pub has_menu_widget: Option<bool>,
    pub header_img: Option<Value>,
    pub header_size: Option<Value>,
    pub header_title: Option<String>,
    pub hide_ads: Option<bool>,
    pub icon_img: Option<String>,
    pub icon_size: Option<Value>,
    pub id: Option<String>,
    pub is_chat_post_feature_enabled: Option<bool>,
    pub is_crosspostable_subreddit: Option<bool>,
    pub is_enrolled_in_new_modmail: Option<bool>,
    pub key_color: Option<String>,
    pub lang: Option<String>,
    pub link_flair_enabled: Option<bool>,
    pub link_flair_position: Option<String>,
    pub mobile_banner_image: Option<String>,
    pub name: Option<String>,
    pub notification_level: Option<String>,
    pub original_content_tag_enabled: Option<bool>,
    pub over18: Option<bool>,
    pub prediction_leaderboard_entry_type: Option<String>,
    pub primary_color: Option<String>,
    pub public_description: Option<String>,
    pub public_description_html: Option<String>,
    pub public_traffic: Option<bool>,
    pub quarantine: Option<bool>,
    pub restrict_commenting: Option<bool>,
    pub restrict_posting: Option<bool>,
    pub show_media: Option<bool>,
    pub show_media_preview: Option<bool>,
    pub spoilers_enabled: Option<bool>,
    pub submission_type: Option<String>,
    pub submit_link_label: Option<String>,
    pub submit_text: Option<String>,
    pub submit_text_html: Option<Value>,
    pub submit_text_label: Option<String>,
    pub subreddit_type: Option<String>,
    pub subscribers: Option<i64>,
    pub suggested_comment_sort: Option<Value>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub user_can_flair_in_sr: Option<bool>,
    pub user_flair_background_color: Option<Value>,
    pub user_flair_css_class: Option<Value>,
    pub user_flair_enabled_in_sr: Option<bool>,
    pub user_flair_position: Option<String>,
    pub user_flair_template_id: Option<Value>,
    pub user_flair_text: Option<Value>,
    pub user_flair_text_color: Option<Value>,
    pub user_flair_type: Option<String>,
    pub user_has_favorited: Option<bool>,
    pub user_is_banned: Option<bool>,
    pub user_is_contributor: Option<bool>,
    pub user_is_moderator: Option<bool>,
    pub user_is_muted: Option<bool>,
    pub user_is_subscriber: Option<bool>,
    pub user_sr_flair_enabled: Option<bool>,
    pub user_sr_theme_enabled: Option<bool>,
    pub whitelist_status: Option<Value>,
    pub wiki_enabled: Option<bool>,
    pub wls: Option<Value>,
    pub kind: Option<String>,
}
impl<'de> Deserialize<'de> for GenericResponse<AboutSubreddit> {
    fn deserialize<D>(deserializer: D) -> Result<GenericResponse<AboutSubreddit>, D::Error>
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
pub type SubredditResponse = GenericResponse<AboutSubreddit>;
pub type Subreddits = GenericListing<AboutSubreddit>;
