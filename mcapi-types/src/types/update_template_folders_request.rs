pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateTemplateFoldersRequest {
    /// The name of the folder.
    #[serde(default)]
    pub name: String,
}

impl UpdateTemplateFoldersRequest {
    pub fn builder() -> UpdateTemplateFoldersRequestBuilder {
        <UpdateTemplateFoldersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateTemplateFoldersRequestBuilder {
    name: Option<String>,
}

impl UpdateTemplateFoldersRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateTemplateFoldersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateTemplateFoldersRequestBuilder::name)
    pub fn build(self) -> Result<UpdateTemplateFoldersRequest, BuildError> {
        Ok(UpdateTemplateFoldersRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

