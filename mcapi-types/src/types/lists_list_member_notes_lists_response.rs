pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The last 10 notes for a specific list member, based on date created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberNotesListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberNotesListsResponseLinksItem>>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// An array of objects, each representing a note resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<MemberNotes>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberNotesListsResponse {
    pub fn builder() -> ListMemberNotesListsResponseBuilder {
        <ListMemberNotesListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberNotesListsResponseBuilder {
    links: Option<Vec<ListMemberNotesListsResponseLinksItem>>,
    email_id: Option<String>,
    list_id: Option<String>,
    notes: Option<Vec<MemberNotes>>,
    total_items: Option<i64>,
}

impl ListMemberNotesListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberNotesListsResponseLinksItem>) -> Self {
        self.links = Some(value);
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

    pub fn notes(mut self, value: Vec<MemberNotes>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberNotesListsResponse`].
    pub fn build(self) -> Result<ListMemberNotesListsResponse, BuildError> {
        Ok(ListMemberNotesListsResponse {
            links: self.links,
            email_id: self.email_id,
            list_id: self.list_id,
            notes: self.notes,
            total_items: self.total_items,
        })
    }
}
