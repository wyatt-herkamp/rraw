pub mod response;

use crate::client::Client;
use crate::comments::CommentRetriever;
use crate::responses::{GenericListing, ListingArray};
use crate::submission::response::SubmissionsResponse;
use crate::utils::options::{CommentOption, FeedOption};
use async_trait::async_trait;

use crate::error::Error;

pub trait SubmissionType<'a>: Sized + Sync + Send {
    fn get_permalink(&self) -> &String;

    fn to_submission(&'a self, me: &'a Client) -> Submission<'a, Self>
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

pub struct Submission<'a, T: SubmissionType<'a>> {
    pub submission: &'a T,
    pub(crate) me: &'a Client,
}

#[async_trait]
impl<'a, T: SubmissionType<'a>> CommentRetriever for Submission<'a, T> {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, Error> {
        let mut path = self.submission.get_permalink().to_string();
        if let Some(options) = sort {
            options.extend(&mut path)
        }
        return self.me.get_json::<ListingArray>(&path, false).await;
    }
}

pub type Submissions<'a, T> = GenericListing<Submission<'a, T>>;

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
