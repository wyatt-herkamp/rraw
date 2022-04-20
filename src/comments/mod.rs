pub mod response;

use crate::me::Me;
use crate::responses::{ListingArray, GenericListing};
use crate::utils::options::{CommentOption};
use async_trait::async_trait;
use crate::comments::response::CommentsResponse;
use crate::utils::error::APIError;

pub trait CommentType<'a>: Sized + Sync + Send {
    fn get_permalink(&self) -> &String;

    fn to_comment(&'a self, me: &'a Me) -> Comment<'a, Self> where Self: CommentType<'a> {
        return Comment {
            comment: self,
            me,
        };
    }
}

impl<'a> CommentType<'a> for String {
    fn get_permalink(&self) -> &String {
        return self;
    }
}

pub struct Comment<'a, T: CommentType<'a>> {
    pub comment:&'a T,
    pub(crate) me: &'a Me,
}

pub type Comments<'a, T> = GenericListing<Comment<'a, T>>;

#[async_trait]
pub trait CommentRetriever {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, APIError>;
}