pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of a subscriber removed from an Automation workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriberRemovedFromAutomationWorkflow {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Vec<SubscriberRemovedFromAutomationWorkflowLinksItemItem>>>,
    /// The list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A string that uniquely identifies a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl SubscriberRemovedFromAutomationWorkflow {
    pub fn builder() -> SubscriberRemovedFromAutomationWorkflowBuilder {
        <SubscriberRemovedFromAutomationWorkflowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriberRemovedFromAutomationWorkflowBuilder {
    links: Option<Vec<Vec<SubscriberRemovedFromAutomationWorkflowLinksItemItem>>>,
    email_address: Option<String>,
    id: Option<String>,
    list_id: Option<String>,
    workflow_id: Option<String>,
}

impl SubscriberRemovedFromAutomationWorkflowBuilder {
    pub fn links(mut self, value: Vec<Vec<SubscriberRemovedFromAutomationWorkflowLinksItemItem>>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriberRemovedFromAutomationWorkflow`].
    pub fn build(self) -> Result<SubscriberRemovedFromAutomationWorkflow, BuildError> {
        Ok(SubscriberRemovedFromAutomationWorkflow {
            links: self.links,
            email_address: self.email_address,
            id: self.id,
            list_id: self.list_id,
            workflow_id: self.workflow_id,
        })
    }
}
