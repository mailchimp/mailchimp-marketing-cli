pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCampaignFoldersRequest {
    /// Name to associate with the folder.
    #[serde(default)]
    pub name: String,
}

impl CreateCampaignFoldersRequest {
    pub fn builder() -> CreateCampaignFoldersRequestBuilder {
        <CreateCampaignFoldersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCampaignFoldersRequestBuilder {
    name: Option<String>,
}

impl CreateCampaignFoldersRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCampaignFoldersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateCampaignFoldersRequestBuilder::name)
    pub fn build(self) -> Result<CreateCampaignFoldersRequest, BuildError> {
        Ok(CreateCampaignFoldersRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

