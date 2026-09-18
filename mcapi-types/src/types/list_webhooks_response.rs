pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Manage webhooks for a specific list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhooksResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListWebhooksResponseLinksItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// An array of objects, each representing a specific list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<ListWebhooks>>,
}

impl ListWebhooksResponse {
    pub fn builder() -> ListWebhooksResponseBuilder {
        <ListWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhooksResponseBuilder {
    links: Option<Vec<ListWebhooksResponseLinksItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
    webhooks: Option<Vec<ListWebhooks>>,
}

impl ListWebhooksResponseBuilder {
    pub fn links(mut self, value: Vec<ListWebhooksResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn webhooks(mut self, value: Vec<ListWebhooks>) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListWebhooksResponse`].
    pub fn build(self) -> Result<ListWebhooksResponse, BuildError> {
        Ok(ListWebhooksResponse {
            links: self.links,
            list_id: self.list_id,
            total_items: self.total_items,
            webhooks: self.webhooks,
        })
    }
}
