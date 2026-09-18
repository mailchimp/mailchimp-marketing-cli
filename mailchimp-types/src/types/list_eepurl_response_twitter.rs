pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of Twitter activity for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEepurlResponseTwitter {
    /// The day and time of the first recorded tweet with a link to the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_tweet: Option<String>,
    /// The day and time of the last recorded tweet with a link to the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_tweet: Option<String>,
    /// The number of retweets that include a link to the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retweets: Option<i64>,
    /// A summary of tweets that include a link to the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<ListEepurlResponseTwitterStatusesItem>>,
    /// The number of tweets including a link to the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tweets: Option<i64>,
}

impl ListEepurlResponseTwitter {
    pub fn builder() -> ListEepurlResponseTwitterBuilder {
        <ListEepurlResponseTwitterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEepurlResponseTwitterBuilder {
    first_tweet: Option<String>,
    last_tweet: Option<String>,
    retweets: Option<i64>,
    statuses: Option<Vec<ListEepurlResponseTwitterStatusesItem>>,
    tweets: Option<i64>,
}

impl ListEepurlResponseTwitterBuilder {
    pub fn first_tweet(mut self, value: impl Into<String>) -> Self {
        self.first_tweet = Some(value.into());
        self
    }

    pub fn last_tweet(mut self, value: impl Into<String>) -> Self {
        self.last_tweet = Some(value.into());
        self
    }

    pub fn retweets(mut self, value: i64) -> Self {
        self.retweets = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<ListEepurlResponseTwitterStatusesItem>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn tweets(mut self, value: i64) -> Self {
        self.tweets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEepurlResponseTwitter`].
    pub fn build(self) -> Result<ListEepurlResponseTwitter, BuildError> {
        Ok(ListEepurlResponseTwitter {
            first_tweet: self.first_tweet,
            last_tweet: self.last_tweet,
            retweets: self.retweets,
            statuses: self.statuses,
            tweets: self.tweets,
        })
    }
}
