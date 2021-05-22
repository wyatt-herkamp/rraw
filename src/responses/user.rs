use crate::responses::{GenericResponse, GenericListing};
pub use serde::Deserialize;

///About Data for the User
#[derive(Deserialize, Debug)]
pub struct AboutUser {
    /// The Reddit generated avatar image
    pub snoovatar_img: Option<String>,
    /// classic icon img
    pub icon_img: Option<String>,
    /// Reddit Username
    pub name: String,
}

/// About with a GenericResponse Wrap
pub type About = GenericResponse<AboutUser>;
/// A listing of user abouts
pub type Users = GenericListing<AboutUser>;