use crate::me::Me;
use crate::utils::error::APIError;
use crate::responses::user::About;

pub struct User<'a> {
    pub(crate) me: &'a Me,
    pub name: String,
}

impl<'a> PartialEq for User<'a> {
    fn eq(&self, other: &User) -> bool {
        self.name == other.name
    }
}

impl<'a> User<'a> {
    pub async fn about(&self) -> Result<About, APIError> {
        let string = format!("/u/{}/about.json", self.name.clone());
        return self.me.get_json::<About>(&*string, false).await;
    }
}