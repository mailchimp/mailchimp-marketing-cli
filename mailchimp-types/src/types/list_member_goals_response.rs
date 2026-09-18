pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The last 50 Goal events for a member on a specific list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberGoalsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberGoalsResponseLinksItem>>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The last 50 Goal events triggered by a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<Vec<ListMemberGoalsResponseGoalsItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberGoalsResponse {
    pub fn builder() -> ListMemberGoalsResponseBuilder {
        <ListMemberGoalsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberGoalsResponseBuilder {
    links: Option<Vec<ListMemberGoalsResponseLinksItem>>,
    email_id: Option<String>,
    goals: Option<Vec<ListMemberGoalsResponseGoalsItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListMemberGoalsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberGoalsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn goals(mut self, value: Vec<ListMemberGoalsResponseGoalsItem>) -> Self {
        self.goals = Some(value);
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

    /// Consumes the builder and constructs a [`ListMemberGoalsResponse`].
    pub fn build(self) -> Result<ListMemberGoalsResponse, BuildError> {
        Ok(ListMemberGoalsResponse {
            links: self.links,
            email_id: self.email_id,
            goals: self.goals,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
