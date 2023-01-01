use crate::responses::{GenericResponse, RedditResponse};
use serde::Deserialize;
use std::fmt::{Debug, Formatter};

#[derive(Deserialize)]
/// The Listing API for async RRAW
pub struct Listing<T> {
    /// Modhash from Reddit
    pub modhash: Option<String>,
    /// After from Reddit
    pub after: Option<String>,
    /// before from Reddit
    pub before: Option<String>,
    /// The Children of the post. Either will be a GenericResponse<T> or A RedditResponse
    pub children: Vec<T>,
}

impl<T: Debug> Debug for Listing<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Listing] Children Available: {}", self.children.len())
    }
}

/// GenericListing mixes the GenericResponse and Listing for simplicity
pub type GenericListing<T> = GenericResponse<Listing<GenericResponse<T>>>;
/// RedditListing uses a RedditResponse
pub type RedditListing = GenericResponse<Listing<RedditResponse>>;
/// Due to a Random Response this is a type that Returns an Array of RedditListings.
pub type ListingArray = Vec<RedditListing>;
