pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateMemberNoteRequest {
    /// The content of the note. Note length is limited to 1,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl UpdateMemberNoteRequest {
    pub fn builder() -> UpdateMemberNoteRequestBuilder {
        <UpdateMemberNoteRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMemberNoteRequestBuilder {
    note: Option<String>,
}

impl UpdateMemberNoteRequestBuilder {
    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateMemberNoteRequest`].
    pub fn build(self) -> Result<UpdateMemberNoteRequest, BuildError> {
        Ok(UpdateMemberNoteRequest {
            note: self.note,
        })
    }
}

