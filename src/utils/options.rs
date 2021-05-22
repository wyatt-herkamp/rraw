pub use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct FeedOption {
    pub after: Option<String>,
    pub before: Option<String>,
    pub count: Option<u32>,
    pub period: Option<TimePeriod>,
}

impl FeedOption {
    pub fn url(self) -> String {
        let mut url = String::new();
        if let Some(after) = self.after {
            url.push_str(&mut format!("&after={}", after));
        }
        if let Some(before) = self.before {
            url.push_str(&mut format!("&before={}", before));
        }

        if let Some(count) = self.count {
            url.push_str(&mut format!("&count={}", count));
        }

        if let Some(period) = self.period {
            url.push_str(&mut format!("&t={}", period.get_string()));
        }
        return url;
    }
}

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
