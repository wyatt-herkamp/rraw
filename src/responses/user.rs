use crate::responses::{GenericResponse, GenericListing};
pub use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct AboutUser {
    pub snoovatar_img: Option<String>,
    pub icon_img: Option<String>,
    pub name: String,
}

pub type About = GenericResponse<AboutUser>;
pub type Users = GenericListing<AboutUser>;