pub mod response;

use crate::auth::Authenticator;
use crate::comments::CommentRetriever;
use crate::submission::response::SubmissionsResponse;
use crate::utils::options::{CommentOption, FeedOption};
use crate::Client;
use async_trait::async_trait;

use crate::error::Error;
use crate::responses::listing::{GenericListing, ListingArray};

pub trait SubmissionType<'a>: Sized + Sync + Send {
    fn get_permalink(&self) -> &String;

    fn to_submission<A: Authenticator>(&'a self, me: &'a Client<A>) -> Submission<'a, A, Self>
    where
        Self: SubmissionType<'a>,
    {
        Submission {
            submission: self,
            me,
        }
    }
}

impl<'a> SubmissionType<'a> for String {
    fn get_permalink(&self) -> &String {
        self
    }
}

pub struct Submission<'a, A: Authenticator, T: SubmissionType<'a>> {
    pub submission: &'a T,
    pub(crate) me: &'a Client<A>,
}

#[async_trait]
impl<'a, A: Authenticator, T: SubmissionType<'a>> CommentRetriever for Submission<'a, A, T> {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, Error> {
        let mut path = self.submission.get_permalink().to_string();
        if let Some(options) = sort {
            options.extend(&mut path)
        }
        return self.me.get_json::<ListingArray>(&path, false).await;
    }
}

pub type Submissions<'a, A, T> = GenericListing<Submission<'a, A, T>>;

#[async_trait]
pub trait SubmissionRetriever {
    async fn get_submissions<T: Into<String> + std::marker::Send>(
        &self,
        sort: T,
        feed_options: Option<FeedOption>,
    ) -> Result<SubmissionsResponse, Error>;

    async fn hot(&self, feed_options: Option<FeedOption>) -> Result<SubmissionsResponse, Error> {
        return self.get_submissions("hot", feed_options).await;
    }
}
