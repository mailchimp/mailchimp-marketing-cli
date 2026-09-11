pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of subscribers who were sent a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSentToReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSentToReportsResponseLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// An array of objects, each representing a campaign recipient.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_to: Option<Vec<SentTo>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSentToReportsResponse {
    pub fn builder() -> ListSentToReportsResponseBuilder {
        <ListSentToReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSentToReportsResponseBuilder {
    links: Option<Vec<ListSentToReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    sent_to: Option<Vec<SentTo>>,
    total_items: Option<i64>,
}

impl ListSentToReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSentToReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn sent_to(mut self, value: Vec<SentTo>) -> Self {
        self.sent_to = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSentToReportsResponse`].
    pub fn build(self) -> Result<ListSentToReportsResponse, BuildError> {
        Ok(ListSentToReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            sent_to: self.sent_to,
            total_items: self.total_items,
        })
    }
}
