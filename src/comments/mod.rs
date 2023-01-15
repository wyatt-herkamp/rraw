pub mod response;

use crate::auth::Authenticator;
use crate::utils::options::CommentOption;
use crate::Client;

use crate::error::Error;
use crate::responses::listing::{GenericListing, ListingArray};

pub trait CommentType<'a>: Sized + Sync + Send {
    fn get_permalink(&self) -> &String;

    fn to_comment<A: Authenticator>(&'a self, me: &'a Client<A>) -> Comment<'a, A, Self>
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

pub struct Comment<'a, A: Authenticator, T: CommentType<'a>> {
    pub comment: &'a T,
    pub(crate) me: &'a Client<A>,
}

pub type Comments<'a, A, T> = GenericListing<Comment<'a, A, T>>;

pub trait CommentRetriever {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, Error>;
}
impl<'a, A: Authenticator, T: CommentType<'a>> CommentRetriever for Comment<'a, A, T> {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, Error> {
        let mut path = self.comment.get_permalink().to_string();
        if let Some(options) = sort {
            options.extend(&mut path)
        }
        return self.me.get_json::<ListingArray>(&path, false, false).await;
    }
}
