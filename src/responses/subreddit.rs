pub use serde::Deserialize;
use crate::responses::GenericResponse;

#[derive(Deserialize, Debug)]
pub struct AboutSubreddit {
    pub title: Option<String>,
    pub display_name: Option<String>,
    pub display_name_prefixed: Option<String>,
    pub url: Option<String>,
    pub public_description: Option<String>,
    pub public_description_html: Option<String>,
    pub community_icon: Option<String>,
    pub description: Option<String>,
    pub description_html: Option<String>,
    pub subscribers: u64,
    pub accounts_active: u64,
    pub active_user_count: u64,

}

pub type About = GenericResponse<AboutSubreddit>;