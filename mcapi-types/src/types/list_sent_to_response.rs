pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of subscribers who were sent a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSentToResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSentToResponseLinksItem>>,
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

impl ListSentToResponse {
    pub fn builder() -> ListSentToResponseBuilder {
        <ListSentToResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSentToResponseBuilder {
    links: Option<Vec<ListSentToResponseLinksItem>>,
    campaign_id: Option<String>,
    sent_to: Option<Vec<SentTo>>,
    total_items: Option<i64>,
}

impl ListSentToResponseBuilder {
    pub fn links(mut self, value: Vec<ListSentToResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListSentToResponse`].
    pub fn build(self) -> Result<ListSentToResponse, BuildError> {
        Ok(ListSentToResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            sent_to: self.sent_to,
            total_items: self.total_items,
        })
    }
}
