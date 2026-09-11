pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Member activity events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberActivityListsResponseActivityItem {
    /// The type of action recorded for the subscriber.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// The web-based ID for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The ID of the parent campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_campaign: Option<String>,
    /// The date and time recorded for the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// If set, the campaign's title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The type of campaign that was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// For clicks, the URL the subscriber clicked on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ListMemberActivityListsResponseActivityItem {
    pub fn builder() -> ListMemberActivityListsResponseActivityItemBuilder {
        <ListMemberActivityListsResponseActivityItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberActivityListsResponseActivityItemBuilder {
    action: Option<String>,
    campaign_id: Option<String>,
    parent_campaign: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
    title: Option<String>,
    r#type: Option<String>,
    url: Option<String>,
}

impl ListMemberActivityListsResponseActivityItemBuilder {
    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn parent_campaign(mut self, value: impl Into<String>) -> Self {
        self.parent_campaign = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListMemberActivityListsResponseActivityItem`].
    pub fn build(self) -> Result<ListMemberActivityListsResponseActivityItem, BuildError> {
        Ok(ListMemberActivityListsResponseActivityItem {
            action: self.action,
            campaign_id: self.campaign_id,
            parent_campaign: self.parent_campaign,
            timestamp: self.timestamp,
            title: self.title,
            r#type: self.r#type,
            url: self.url,
        })
    }
}
