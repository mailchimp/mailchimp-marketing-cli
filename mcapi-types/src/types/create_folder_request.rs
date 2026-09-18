pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFolderRequest {
    /// The name of the folder.
    #[serde(default)]
    pub name: String,
}

impl CreateFolderRequest {
    pub fn builder() -> CreateFolderRequestBuilder {
        <CreateFolderRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFolderRequestBuilder {
    name: Option<String>,
}

impl CreateFolderRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFolderRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateFolderRequestBuilder::name)
    pub fn build(self) -> Result<CreateFolderRequest, BuildError> {
        Ok(CreateFolderRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

