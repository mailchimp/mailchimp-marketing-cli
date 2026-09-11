pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemberNoteListsRequest {
    /// The content of the note. Note length is limited to 1,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl CreateMemberNoteListsRequest {
    pub fn builder() -> CreateMemberNoteListsRequestBuilder {
        <CreateMemberNoteListsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberNoteListsRequestBuilder {
    note: Option<String>,
}

impl CreateMemberNoteListsRequestBuilder {
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberNoteListsRequest`].
    pub fn build(self) -> Result<CreateMemberNoteListsRequest, BuildError> {
        Ok(CreateMemberNoteListsRequest {
            note: self.note,
        })
    }
}

