pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTemplateFoldersRequest {
    /// The name of the folder.
    #[serde(default)]
    pub name: String,
}

impl CreateTemplateFoldersRequest {
    pub fn builder() -> CreateTemplateFoldersRequestBuilder {
        <CreateTemplateFoldersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTemplateFoldersRequestBuilder {
    name: Option<String>,
}

impl CreateTemplateFoldersRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateTemplateFoldersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateTemplateFoldersRequestBuilder::name)
    pub fn build(self) -> Result<CreateTemplateFoldersRequest, BuildError> {
        Ok(CreateTemplateFoldersRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

