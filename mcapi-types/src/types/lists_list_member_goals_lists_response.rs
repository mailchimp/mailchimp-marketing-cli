pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The last 50 Goal events for a member on a specific list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberGoalsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberGoalsListsResponseLinksItem>>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The last 50 Goal events triggered by a member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goals: Option<Vec<ListMemberGoalsListsResponseGoalsItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberGoalsListsResponse {
    pub fn builder() -> ListMemberGoalsListsResponseBuilder {
        <ListMemberGoalsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberGoalsListsResponseBuilder {
    links: Option<Vec<ListMemberGoalsListsResponseLinksItem>>,
    email_id: Option<String>,
    goals: Option<Vec<ListMemberGoalsListsResponseGoalsItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListMemberGoalsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberGoalsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn goals(mut self, value: Vec<ListMemberGoalsListsResponseGoalsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListMemberGoalsListsResponse`].
    pub fn build(self) -> Result<ListMemberGoalsListsResponse, BuildError> {
        Ok(ListMemberGoalsListsResponse {
            links: self.links,
            email_id: self.email_id,
            goals: self.goals,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
