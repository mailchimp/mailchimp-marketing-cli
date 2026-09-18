pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemberNoteRequest {
    /// The content of the note. Note length is limited to 1,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl CreateMemberNoteRequest {
    pub fn builder() -> CreateMemberNoteRequestBuilder {
        <CreateMemberNoteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemberNoteRequestBuilder {
    note: Option<String>,
}

impl CreateMemberNoteRequestBuilder {
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMemberNoteRequest`].
    pub fn build(self) -> Result<CreateMemberNoteRequest, BuildError> {
        Ok(CreateMemberNoteRequest {
            note: self.note,
        })
    }
}

