pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The most recent Note added about this member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMembersLastNote {
    /// The date and time the note was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The author of the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The content of the note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// The note id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_id: Option<i64>,
}

impl ListMembersLastNote {
    pub fn builder() -> ListMembersLastNoteBuilder {
        <ListMembersLastNoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMembersLastNoteBuilder {
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    note: Option<String>,
    note_id: Option<i64>,
}

impl ListMembersLastNoteBuilder {
    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn note_id(mut self, value: i64) -> Self {
        self.note_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMembersLastNote`].
    pub fn build(self) -> Result<ListMembersLastNote, BuildError> {
        Ok(ListMembersLastNote {
            created_at: self.created_at,
            created_by: self.created_by,
            note: self.note,
            note_id: self.note_id,
        })
    }
}
