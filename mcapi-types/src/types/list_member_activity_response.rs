pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The last 50 member events for a list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberActivityResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberActivityResponseLinksItem>>,
    /// An array of objects, each representing a member event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Vec<ListMemberActivityResponseActivityItem>>,
    /// As Mailchimp evolves beyond email, you may eventually have contacts without email addresses. While the `email_id` is the MD5 hash of their email address, this `contact_id` is agnostic of contact’s inclusion of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberActivityResponse {
    pub fn builder() -> ListMemberActivityResponseBuilder {
        <ListMemberActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberActivityResponseBuilder {
    links: Option<Vec<ListMemberActivityResponseLinksItem>>,
    activity: Option<Vec<ListMemberActivityResponseActivityItem>>,
    contact_id: Option<String>,
    email_id: Option<String>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListMemberActivityResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberActivityResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn activity(mut self, value: Vec<ListMemberActivityResponseActivityItem>) -> Self {
        self.activity = Some(value);
        self
    }

    pub fn contact_id(mut self, value: impl Into<String>) -> Self {
        self.contact_id = Some(value.into());
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListMemberActivityResponse`].
    pub fn build(self) -> Result<ListMemberActivityResponse, BuildError> {
        Ok(ListMemberActivityResponse {
            links: self.links,
            activity: self.activity,
            contact_id: self.contact_id,
            email_id: self.email_id,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
