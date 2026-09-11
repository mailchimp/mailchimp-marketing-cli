pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of the subscribers who were removed from an Automation workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListRemovedSubscribersAutomationsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Vec<ListRemovedSubscribersAutomationsResponseLinksItemItem>>>,
    /// An array of objects, each representing a subscriber who was removed from an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<SubscriberRemovedFromAutomationWorkflow>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl ListRemovedSubscribersAutomationsResponse {
    pub fn builder() -> ListRemovedSubscribersAutomationsResponseBuilder {
        <ListRemovedSubscribersAutomationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListRemovedSubscribersAutomationsResponseBuilder {
    links: Option<Vec<Vec<ListRemovedSubscribersAutomationsResponseLinksItemItem>>>,
    subscribers: Option<Vec<SubscriberRemovedFromAutomationWorkflow>>,
    total_items: Option<i64>,
    workflow_id: Option<String>,
}

impl ListRemovedSubscribersAutomationsResponseBuilder {
    pub fn links(mut self, value: Vec<Vec<ListRemovedSubscribersAutomationsResponseLinksItemItem>>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn subscribers(mut self, value: Vec<SubscriberRemovedFromAutomationWorkflow>) -> Self {
        self.subscribers = Some(value);
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

    /// Consumes the builder and constructs a [`ListRemovedSubscribersAutomationsResponse`].
    pub fn build(self) -> Result<ListRemovedSubscribersAutomationsResponse, BuildError> {
        Ok(ListRemovedSubscribersAutomationsResponse {
            links: self.links,
            subscribers: self.subscribers,
            total_items: self.total_items,
            workflow_id: self.workflow_id,
        })
    }
}
