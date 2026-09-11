pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Manage members of a specific Mailchimp list, including currently subscribed, unsubscribed, and bounced members.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMembersListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMembersListsResponseLinksItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// An array of objects, each representing a specific list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ListMembers>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMembersListsResponse {
    pub fn builder() -> ListMembersListsResponseBuilder {
        <ListMembersListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersListsResponseBuilder {
    links: Option<Vec<ListMembersListsResponseLinksItem>>,
    list_id: Option<String>,
    members: Option<Vec<ListMembers>>,
    total_items: Option<i64>,
}

impl ListMembersListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMembersListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<ListMembers>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMembersListsResponse`].
    pub fn build(self) -> Result<ListMembersListsResponse, BuildError> {
        Ok(ListMembersListsResponse {
            links: self.links,
            list_id: self.list_id,
            members: self.members,
            total_items: self.total_items,
        })
    }
}
