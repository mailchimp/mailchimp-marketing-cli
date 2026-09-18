pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Manage members of a specific Mailchimp list, including currently subscribed, unsubscribed, and bounced members.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMembersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMembersResponseLinksItem>>,
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

impl ListMembersResponse {
    pub fn builder() -> ListMembersResponseBuilder {
        <ListMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersResponseBuilder {
    links: Option<Vec<ListMembersResponseLinksItem>>,
    list_id: Option<String>,
    members: Option<Vec<ListMembers>>,
    total_items: Option<i64>,
}

impl ListMembersResponseBuilder {
    pub fn links(mut self, value: Vec<ListMembersResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListMembersResponse`].
    pub fn build(self) -> Result<ListMembersResponse, BuildError> {
        Ok(ListMembersResponse {
            links: self.links,
            list_id: self.list_id,
            members: self.members,
            total_items: self.total_items,
        })
    }
}
