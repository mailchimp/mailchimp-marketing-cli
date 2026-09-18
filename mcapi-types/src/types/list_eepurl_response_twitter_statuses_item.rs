pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An individual tweet.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEepurlResponseTwitterStatusesItem {
    /// A timestamp for the tweet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub datetime: Option<DateTime<FixedOffset>>,
    /// A 'true' or 'false' status of whether the tweet is a retweet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_retweet: Option<bool>,
    /// The Twitter handle for the author of the tweet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_name: Option<String>,
    /// The body of the tweet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The individual id for the tweet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
}

impl ListEepurlResponseTwitterStatusesItem {
    pub fn builder() -> ListEepurlResponseTwitterStatusesItemBuilder {
        <ListEepurlResponseTwitterStatusesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEepurlResponseTwitterStatusesItemBuilder {
    datetime: Option<DateTime<FixedOffset>>,
    is_retweet: Option<bool>,
    screen_name: Option<String>,
    status: Option<String>,
    status_id: Option<String>,
}

impl ListEepurlResponseTwitterStatusesItemBuilder {
    pub fn datetime(mut self, value: DateTime<FixedOffset>) -> Self {
        self.datetime = Some(value);
        self
    }

    pub fn is_retweet(mut self, value: bool) -> Self {
        self.is_retweet = Some(value);
        self
    }

    pub fn screen_name(mut self, value: impl Into<String>) -> Self {
        self.screen_name = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn status_id(mut self, value: impl Into<String>) -> Self {
        self.status_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEepurlResponseTwitterStatusesItem`].
    pub fn build(self) -> Result<ListEepurlResponseTwitterStatusesItem, BuildError> {
        Ok(ListEepurlResponseTwitterStatusesItem {
            datetime: self.datetime,
            is_retweet: self.is_retweet,
            screen_name: self.screen_name,
            status: self.status,
            status_id: self.status_id,
        })
    }
}
