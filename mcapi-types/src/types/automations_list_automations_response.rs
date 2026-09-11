pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An array of objects, each representing an Automation workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAutomationsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAutomationsResponseLinksItem>>,
    /// An array of objects, each representing an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automations: Option<Vec<AutomationWorkflow>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListAutomationsResponse {
    pub fn builder() -> ListAutomationsResponseBuilder {
        <ListAutomationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAutomationsResponseBuilder {
    links: Option<Vec<ListAutomationsResponseLinksItem>>,
    automations: Option<Vec<AutomationWorkflow>>,
    total_items: Option<i64>,
}

impl ListAutomationsResponseBuilder {
    pub fn links(mut self, value: Vec<ListAutomationsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn automations(mut self, value: Vec<AutomationWorkflow>) -> Self {
        self.automations = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAutomationsResponse`].
    pub fn build(self) -> Result<ListAutomationsResponse, BuildError> {
        Ok(ListAutomationsResponse {
            links: self.links,
            automations: self.automations,
            total_items: self.total_items,
        })
    }
}
