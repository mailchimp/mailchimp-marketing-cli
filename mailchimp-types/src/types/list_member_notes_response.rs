pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The last 10 notes for a specific list member, based on date created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberNotesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberNotesResponseLinksItem>>,
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

impl ListMemberNotesResponse {
    pub fn builder() -> ListMemberNotesResponseBuilder {
        <ListMemberNotesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberNotesResponseBuilder {
    links: Option<Vec<ListMemberNotesResponseLinksItem>>,
    email_id: Option<String>,
    list_id: Option<String>,
    notes: Option<Vec<MemberNotes>>,
    total_items: Option<i64>,
}

impl ListMemberNotesResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberNotesResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListMemberNotesResponse`].
    pub fn build(self) -> Result<ListMemberNotesResponse, BuildError> {
        Ok(ListMemberNotesResponse {
            links: self.links,
            email_id: self.email_id,
            list_id: self.list_id,
            notes: self.notes,
            total_items: self.total_items,
        })
    }
}
