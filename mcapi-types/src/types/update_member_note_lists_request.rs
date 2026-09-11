pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMemberNoteListsRequest {
    /// The content of the note. Note length is limited to 1,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl UpdateMemberNoteListsRequest {
    pub fn builder() -> UpdateMemberNoteListsRequestBuilder {
        <UpdateMemberNoteListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberNoteListsRequestBuilder {
    note: Option<String>,
}

impl UpdateMemberNoteListsRequestBuilder {
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMemberNoteListsRequest`].
    pub fn build(self) -> Result<UpdateMemberNoteListsRequest, BuildError> {
        Ok(UpdateMemberNoteListsRequest {
            note: self.note,
        })
    }
}

