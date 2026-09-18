pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of members who have unsubscribed from a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListUnsubscribedResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListUnsubscribedResponseLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// An array of objects, each representing a member who unsubscribed from a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribes: Option<Vec<Unsubscribes>>,
}

impl ListUnsubscribedResponse {
    pub fn builder() -> ListUnsubscribedResponseBuilder {
        <ListUnsubscribedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListUnsubscribedResponseBuilder {
    links: Option<Vec<ListUnsubscribedResponseLinksItem>>,
    campaign_id: Option<String>,
    total_items: Option<i64>,
    unsubscribes: Option<Vec<Unsubscribes>>,
}

impl ListUnsubscribedResponseBuilder {
    pub fn links(mut self, value: Vec<ListUnsubscribedResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn unsubscribes(mut self, value: Vec<Unsubscribes>) -> Self {
        self.unsubscribes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListUnsubscribedResponse`].
    pub fn build(self) -> Result<ListUnsubscribedResponse, BuildError> {
        Ok(ListUnsubscribedResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            total_items: self.total_items,
            unsubscribes: self.unsubscribes,
        })
    }
}
