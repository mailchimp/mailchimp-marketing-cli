pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about subscribers in an Automation email queue.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriberInAutomationQueue {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Vec<SubscriberInAutomationQueueLinksItemItem>>>,
    /// The list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// A string that uniquely identifies an email in an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// A string that uniquely identifies a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// The date and time of the next send for the workflow email in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub next_send: Option<DateTime<FixedOffset>>,
    /// A string that uniquely identifies an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

impl SubscriberInAutomationQueue {
    pub fn builder() -> SubscriberInAutomationQueueBuilder {
        <SubscriberInAutomationQueueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriberInAutomationQueueBuilder {
    links: Option<Vec<Vec<SubscriberInAutomationQueueLinksItemItem>>>,
    email_address: Option<String>,
    email_id: Option<String>,
    id: Option<String>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    next_send: Option<DateTime<FixedOffset>>,
    workflow_id: Option<String>,
}

impl SubscriberInAutomationQueueBuilder {
    pub fn links(mut self, value: Vec<Vec<SubscriberInAutomationQueueLinksItemItem>>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
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

    pub fn list_is_active(mut self, value: bool) -> Self {
        self.list_is_active = Some(value);
        self
    }

    pub fn next_send(mut self, value: DateTime<FixedOffset>) -> Self {
        self.next_send = Some(value);
        self
    }

    pub fn workflow_id(mut self, value: impl Into<String>) -> Self {
        self.workflow_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriberInAutomationQueue`].
    pub fn build(self) -> Result<SubscriberInAutomationQueue, BuildError> {
        Ok(SubscriberInAutomationQueue {
            links: self.links,
            email_address: self.email_address,
            email_id: self.email_id,
            id: self.id,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            next_send: self.next_send,
            workflow_id: self.workflow_id,
        })
    }
}
