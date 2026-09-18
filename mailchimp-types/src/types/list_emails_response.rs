pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of the emails in an Automation workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEmailsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<Vec<ListEmailsResponseLinksItemItem>>>,
    /// An array of objects, each representing an email in an Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<AutomationWorkflowEmail>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListEmailsResponse {
    pub fn builder() -> ListEmailsResponseBuilder {
        <ListEmailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEmailsResponseBuilder {
    links: Option<Vec<Vec<ListEmailsResponseLinksItemItem>>>,
    emails: Option<Vec<AutomationWorkflowEmail>>,
    total_items: Option<i64>,
}

impl ListEmailsResponseBuilder {
    pub fn links(mut self, value: Vec<Vec<ListEmailsResponseLinksItemItem>>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn emails(mut self, value: Vec<AutomationWorkflowEmail>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEmailsResponse`].
    pub fn build(self) -> Result<ListEmailsResponse, BuildError> {
        Ok(ListEmailsResponse {
            links: self.links,
            emails: self.emails,
            total_items: self.total_items,
        })
    }
}
