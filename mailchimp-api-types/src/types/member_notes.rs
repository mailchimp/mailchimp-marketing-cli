pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A specific note for a specific member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemberNotes {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<MemberNotesLinksItem>>,
    /// As Mailchimp evolves beyond email, you may eventually have contacts without email addresses. While the `email_id` is the MD5 hash of their email address, this `contact_id` is agnostic of contact’s inclusion of an email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// The date and time the note was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The author of the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The MD5 hash of the lowercase version of the list member's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_id: Option<String>,
    /// The note id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The content of the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The date and time the note was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl MemberNotes {
    pub fn builder() -> MemberNotesBuilder {
        <MemberNotesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberNotesBuilder {
    links: Option<Vec<MemberNotesLinksItem>>,
    contact_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    email_id: Option<String>,
    id: Option<i64>,
    list_id: Option<String>,
    note: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl MemberNotesBuilder {
    pub fn links(mut self, value: Vec<MemberNotesLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn contact_id(mut self, value: impl Into<String>) -> Self {
        self.contact_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn email_id(mut self, value: impl Into<String>) -> Self {
        self.email_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemberNotes`].
    pub fn build(self) -> Result<MemberNotes, BuildError> {
        Ok(MemberNotes {
            links: self.links,
            contact_id: self.contact_id,
            created_at: self.created_at,
            created_by: self.created_by,
            email_id: self.email_id,
            id: self.id,
            list_id: self.list_id,
            note: self.note,
            updated_at: self.updated_at,
        })
    }
}
