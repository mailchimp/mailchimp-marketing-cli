pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A folder used to organize campaigns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetCampaignFoldersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<GetCampaignFoldersResponseLinksItem>>,
    /// The number of campaigns in the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// A string that uniquely identifies this campaign folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl GetCampaignFoldersResponse {
    pub fn builder() -> GetCampaignFoldersResponseBuilder {
        <GetCampaignFoldersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetCampaignFoldersResponseBuilder {
    links: Option<Vec<GetCampaignFoldersResponseLinksItem>>,
    count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
}

impl GetCampaignFoldersResponseBuilder {
    pub fn links(mut self, value: Vec<GetCampaignFoldersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetCampaignFoldersResponse`].
    pub fn build(self) -> Result<GetCampaignFoldersResponse, BuildError> {
        Ok(GetCampaignFoldersResponse {
            links: self.links,
            count: self.count,
            id: self.id,
            name: self.name,
        })
    }
}
