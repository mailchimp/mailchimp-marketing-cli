pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCampaignFoldersRequest {
    /// Name to associate with the folder.
    #[serde(default)]
    pub name: String,
}

impl UpdateCampaignFoldersRequest {
    pub fn builder() -> UpdateCampaignFoldersRequestBuilder {
        <UpdateCampaignFoldersRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCampaignFoldersRequestBuilder {
    name: Option<String>,
}

impl UpdateCampaignFoldersRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCampaignFoldersRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateCampaignFoldersRequestBuilder::name)
    pub fn build(self) -> Result<UpdateCampaignFoldersRequest, BuildError> {
        Ok(UpdateCampaignFoldersRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

