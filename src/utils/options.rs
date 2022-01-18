use core::fmt;
use std::fmt::{Display, Formatter};

pub use serde::Serialize;

///A simple object to let you set informationons about the listing you are getting
#[derive(Clone, Debug, Serialize)]
pub struct FeedOption {
    pub after: Option<String>,
    pub before: Option<String>,
    pub count: Option<u32>,
    pub period: Option<TimePeriod>,
}

impl FeedOption {
    ///Returns the URL extension for the request
    pub fn url(&self) -> String {
        let mut url = String::new();
        if let Some(after) = &self.after {
            url.push_str(&format!("&after={after}"));
        }
        if let Some(before) = &self.before {
            url.push_str(&format!("&before={before}"));
        }

        if let Some(count) = &self.count {
            url.push_str(&format!("&count={count}"));
        }

        if let Some(period) = &self.period {
            url.push_str(&format!("&t={period}"));
        }
        url
    }
    pub fn extend(&self, value: &mut String) {
        value.push('?');
        value.push_str(self.url().as_str());
    }
}

///Time Period for the request
#[derive(Copy, Clone, Debug, Serialize)]
pub enum TimePeriod {
    Now,
    Today,
    Week,
    Month,
    Year,
    AllTime,
}

impl TimePeriod {
    /// Gets the string for Reddit
    pub fn get_string(&self) -> &str {
        match self {
            TimePeriod::Now => "now",
            TimePeriod::Today => "day",
            TimePeriod::Week => "week",
            TimePeriod::Month => "month",
            TimePeriod::Year => "year",
            TimePeriod::AllTime => "all",
        }
    }
}

impl Display for TimePeriod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            TimePeriod::Now => "now",
            TimePeriod::Today => "day",
            TimePeriod::Week => "week",
            TimePeriod::Month => "month",
            TimePeriod::Year => "year",
            TimePeriod::AllTime => "all",
        };
        write!(f, "{}", string)
    }
}

/// FriendType
pub enum FriendType {
    /// Contributor
    Contributor,
    /// Moderator
    Moderator,
    /// This exist if the reddit api changes in the future or I am missing features
    Custom(String),
}

impl Display for FriendType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let string = match self {
            FriendType::Contributor => "contributor",
            FriendType::Moderator => "moderator",
            FriendType::Custom(str) => str.as_str(),
        };
        write!(f, "{}", string)
    }
}
