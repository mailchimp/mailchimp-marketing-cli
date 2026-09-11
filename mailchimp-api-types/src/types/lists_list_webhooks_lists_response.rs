pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Manage webhooks for a specific list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListWebhooksListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListWebhooksListsResponseLinksItem>>,
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

impl ListWebhooksListsResponse {
    pub fn builder() -> ListWebhooksListsResponseBuilder {
        <ListWebhooksListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListWebhooksListsResponseBuilder {
    links: Option<Vec<ListWebhooksListsResponseLinksItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
    webhooks: Option<Vec<ListWebhooks>>,
}

impl ListWebhooksListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListWebhooksListsResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListWebhooksListsResponse`].
    pub fn build(self) -> Result<ListWebhooksListsResponse, BuildError> {
        Ok(ListWebhooksListsResponse {
            links: self.links,
            list_id: self.list_id,
            total_items: self.total_items,
            webhooks: self.webhooks,
        })
    }
}
