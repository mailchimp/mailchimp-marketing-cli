pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFileRequest {
    /// The base64-encoded contents of the file.
    #[serde(default)]
    pub file_data: String,
    /// The id of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    /// The name of the file.
    #[serde(default)]
    pub name: String,
}

impl CreateFileRequest {
    pub fn builder() -> CreateFileRequestBuilder {
        <CreateFileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFileRequestBuilder {
    file_data: Option<String>,
    folder_id: Option<i64>,
    name: Option<String>,
}

impl CreateFileRequestBuilder {
    pub fn file_data(mut self, value: impl Into<String>) -> Self {
        self.file_data = Some(value.into());
        self
    }

    pub fn folder_id(mut self, value: i64) -> Self {
        self.folder_id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_data`](CreateFileRequestBuilder::file_data)
    /// - [`name`](CreateFileRequestBuilder::name)
    pub fn build(self) -> Result<CreateFileRequest, BuildError> {
        Ok(CreateFileRequest {
            file_data: self.file_data.ok_or_else(|| BuildError::missing_field("file_data"))?,
            folder_id: self.folder_id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

