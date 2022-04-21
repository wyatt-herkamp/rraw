pub mod response;

use crate::client::Client;
use crate::responses::{GenericListing, ListingArray};
use crate::utils::options::CommentOption;
use async_trait::async_trait;

use crate::error::Error;

pub trait CommentType<'a>: Sized + Sync + Send {
    fn get_permalink(&self) -> &String;

    fn to_comment(&'a self, me: &'a Client) -> Comment<'a, Self>
    where
        Self: CommentType<'a>,
    {
        Comment { comment: self, me }
    }
}

impl<'a> CommentType<'a> for String {
    fn get_permalink(&self) -> &String {
        self
    }
}

pub struct Comment<'a, T: CommentType<'a>> {
    pub comment: &'a T,
    pub(crate) me: &'a Client,
}

pub type Comments<'a, T> = GenericListing<Comment<'a, T>>;

#[async_trait]
pub trait CommentRetriever {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, Error>;
}
#[async_trait]
impl<'a, T: CommentType<'a>> CommentRetriever for Comment<'a, T> {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, Error> {
        let mut path = self.comment.get_permalink().to_string();
        if let Some(options) = sort {
            options.extend(&mut path)
        }
        return self.me.get_json::<ListingArray>(&path, false).await;
    }
}
