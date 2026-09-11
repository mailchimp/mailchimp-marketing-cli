pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFolderFileManagerRequest {
    /// The name of the folder.
    #[serde(default)]
    pub name: String,
}

impl CreateFolderFileManagerRequest {
    pub fn builder() -> CreateFolderFileManagerRequestBuilder {
        <CreateFolderFileManagerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFolderFileManagerRequestBuilder {
    name: Option<String>,
}

impl CreateFolderFileManagerRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFolderFileManagerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateFolderFileManagerRequestBuilder::name)
    pub fn build(self) -> Result<CreateFolderFileManagerRequest, BuildError> {
        Ok(CreateFolderFileManagerRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

