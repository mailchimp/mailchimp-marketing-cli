pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateFolderFileManagerRequest {
    /// The name of the folder.
    #[serde(default)]
    pub name: String,
}

impl UpdateFolderFileManagerRequest {
    pub fn builder() -> UpdateFolderFileManagerRequestBuilder {
        <UpdateFolderFileManagerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFolderFileManagerRequestBuilder {
    name: Option<String>,
}

impl UpdateFolderFileManagerRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateFolderFileManagerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateFolderFileManagerRequestBuilder::name)
    pub fn build(self) -> Result<UpdateFolderFileManagerRequest, BuildError> {
        Ok(UpdateFolderFileManagerRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

