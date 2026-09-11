pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Manage webhooks for batch requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBatchWebhooksResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListBatchWebhooksResponseLinksItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// An array of objects, each representing a Batch Webhook.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<BatchWebhook>>,
}

impl ListBatchWebhooksResponse {
    pub fn builder() -> ListBatchWebhooksResponseBuilder {
        <ListBatchWebhooksResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBatchWebhooksResponseBuilder {
    links: Option<Vec<ListBatchWebhooksResponseLinksItem>>,
    total_items: Option<i64>,
    webhooks: Option<Vec<BatchWebhook>>,
}

impl ListBatchWebhooksResponseBuilder {
    pub fn links(mut self, value: Vec<ListBatchWebhooksResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn webhooks(mut self, value: Vec<BatchWebhook>) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBatchWebhooksResponse`].
    pub fn build(self) -> Result<ListBatchWebhooksResponse, BuildError> {
        Ok(ListBatchWebhooksResponse {
            links: self.links,
            total_items: self.total_items,
            webhooks: self.webhooks,
        })
    }
}
