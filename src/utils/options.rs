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
            url.push_str(&mut format!("&after={}", after));
        }
        if let Some(before) = &self.before {
            url.push_str(&mut format!("&before={}", before));
        }

        if let Some(count) = &self.count {
            url.push_str(&mut format!("&count={}", count));
        }

        if let Some(period) = &self.period {
            url.push_str(&mut format!("&t={}", period.get_string()));
        }
        return url;
    }
    pub fn extend(&self, value: &mut String) {
        value.push_str("?");
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
