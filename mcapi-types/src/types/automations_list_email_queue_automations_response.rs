pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An automation workflow
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEmailQueueAutomationsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Vec<ListEmailQueueAutomationsResponseLinksItemItem>>>,
    /// A string that uniquely identifies an email in an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// An array of objects, each representing a subscriber queue for an email in an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Vec<ListEmailQueueAutomationsResponseQueueItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl ListEmailQueueAutomationsResponse {
    pub fn builder() -> ListEmailQueueAutomationsResponseBuilder {
        <ListEmailQueueAutomationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEmailQueueAutomationsResponseBuilder {
    links: Option<Vec<Vec<ListEmailQueueAutomationsResponseLinksItemItem>>>,
    email_id: Option<String>,
    queue: Option<Vec<ListEmailQueueAutomationsResponseQueueItem>>,
    total_items: Option<i64>,
    workflow_id: Option<String>,
}

impl ListEmailQueueAutomationsResponseBuilder {
    pub fn links(mut self, value: Vec<Vec<ListEmailQueueAutomationsResponseLinksItemItem>>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn queue(mut self, value: Vec<ListEmailQueueAutomationsResponseQueueItem>) -> Self {
        self.queue = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEmailQueueAutomationsResponse`].
    pub fn build(self) -> Result<ListEmailQueueAutomationsResponse, BuildError> {
        Ok(ListEmailQueueAutomationsResponse {
            links: self.links,
            email_id: self.email_id,
            queue: self.queue,
            total_items: self.total_items,
            workflow_id: self.workflow_id,
        })
    }
}
