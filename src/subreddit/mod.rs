use crate::me::Me;
use crate::responses::subreddit::{AboutSubreddit, About};
use crate::utils::error::APIError;

pub struct Subreddit<'a> {
    pub(crate) me: &'a Me,
    pub name: String,
}

impl<'a> PartialEq for Subreddit<'a> {
    fn eq(&self, other: &Subreddit) -> bool {
        self.name == other.name
    }
}

impl<'a> Subreddit<'a> {
    pub async fn about(&self) -> Result<About, APIError> {
        let string = format!("/r/{}/about.json", self.name.clone());
        return self.me.get_json::<About>(&*string, false).await;
    }
}