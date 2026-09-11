pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A Chimp Chatter message
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChimpChatterActivityFeedResponseChimpChatterItem {
    /// If it exists, campaign ID for the associated campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// If it exists, list ID for the associated list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListChimpChatterActivityFeedResponseChimpChatterItemType>,
    /// The date and time this activity was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub update_time: Option<DateTime<FixedOffset>>,
    /// URL to a report that includes this activity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListChimpChatterActivityFeedResponseChimpChatterItem {
    pub fn builder() -> ListChimpChatterActivityFeedResponseChimpChatterItemBuilder {
        <ListChimpChatterActivityFeedResponseChimpChatterItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChimpChatterActivityFeedResponseChimpChatterItemBuilder {
    campaign_id: Option<String>,
    list_id: Option<String>,
    message: Option<String>,
    title: Option<String>,
    r#type: Option<ListChimpChatterActivityFeedResponseChimpChatterItemType>,
    update_time: Option<DateTime<FixedOffset>>,
    url: Option<String>,
}

impl ListChimpChatterActivityFeedResponseChimpChatterItemBuilder {
    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ListChimpChatterActivityFeedResponseChimpChatterItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn update_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.update_time = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListChimpChatterActivityFeedResponseChimpChatterItem`].
    pub fn build(self) -> Result<ListChimpChatterActivityFeedResponseChimpChatterItem, BuildError> {
        Ok(ListChimpChatterActivityFeedResponseChimpChatterItem {
            campaign_id: self.campaign_id,
            list_id: self.list_id,
            message: self.message,
            title: self.title,
            r#type: self.r#type,
            update_time: self.update_time,
            url: self.url,
        })
    }
}
