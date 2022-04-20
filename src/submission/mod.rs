pub mod response;

use crate::me::Me;
use crate::responses::{ListingArray, GenericListing};
use crate::submission::response::{SubmissionResponse, SubmissionsResponse};
use crate::utils::options::{CommentOption, FeedOption};
use async_trait::async_trait;
use crate::comments::CommentRetriever;
use crate::comments::response::CommentsResponse;
use crate::submission;
use crate::utils::error::APIError;

pub trait SubmissionType<'a>: Sized + Sync + Send {
    fn get_permalink(&self) -> &String;

    fn to_submission(&'a self, me: &'a Me) -> Submission<'a,Self> where Self: SubmissionType<'a>{
        return Submission {
            submission: &self,
            me,
        };
    }
}

impl<'a> SubmissionType<'a> for String {
    fn get_permalink(&self) -> &String {
        return self;
    }
}

pub struct Submission<'a, T: SubmissionType<'a>> {
    pub submission: &'a T,
    pub(crate) me: &'a Me,
}

#[async_trait]
impl<'a, T: SubmissionType<'a>> CommentRetriever for Submission<'a, T> {
    async fn get_comments(&self, sort: Option<CommentOption>) -> Result<ListingArray, APIError> {
        let mut path = format!("{}", self.submission.get_permalink());
        if let Some(options) = sort {
            options.extend(&mut path)
        }
        return self.me.get_json::<ListingArray>(&path, false).await;
    }
}

pub type Submissions<'a, T> = GenericListing<Submission<'a, T>>;

#[async_trait]
pub trait SubmissionRetriever {
    async fn get_submissions<T: Into<String> + std::marker::Send>(&self, sort: T, feed_options: Option<FeedOption>) -> Result<SubmissionsResponse, APIError>;

    async fn hot(&self, feed_options: Option<FeedOption>) -> Result<SubmissionsResponse, APIError> {
        return self.get_submissions("hot", feed_options).await;
    }
}
