pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateFolderRequest {
    /// The name of the folder.
    #[serde(default)]
    pub name: String,
}

impl UpdateFolderRequest {
    pub fn builder() -> UpdateFolderRequestBuilder {
        <UpdateFolderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFolderRequestBuilder {
    name: Option<String>,
}

impl UpdateFolderRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateFolderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateFolderRequestBuilder::name)
    pub fn build(self) -> Result<UpdateFolderRequest, BuildError> {
        Ok(UpdateFolderRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

