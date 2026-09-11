pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateFileFileManagerRequest {
    /// The id of the folder. Setting `folder_id` to `0` will remove a file from its current folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    /// The name of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdateFileFileManagerRequest {
    pub fn builder() -> UpdateFileFileManagerRequestBuilder {
        <UpdateFileFileManagerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFileFileManagerRequestBuilder {
    folder_id: Option<i64>,
    name: Option<String>,
}

impl UpdateFileFileManagerRequestBuilder {
    pub fn folder_id(mut self, value: i64) -> Self {
        self.folder_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateFileFileManagerRequest`].
    pub fn build(self) -> Result<UpdateFileFileManagerRequest, BuildError> {
        Ok(UpdateFileFileManagerRequest {
            folder_id: self.folder_id,
            name: self.name,
        })
    }
}

