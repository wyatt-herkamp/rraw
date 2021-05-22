pub use serde::Deserialize;
use crate::responses::GenericResponse;

#[derive(Deserialize, Debug)]
pub struct AboutUser {
    pub snoovatar_img: Option<String>,
    pub icon_img: Option<String>,
    pub name: String,
}

pub type About = GenericResponse<AboutUser>;