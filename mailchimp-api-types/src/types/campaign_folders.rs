pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of campaign folders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignFolders {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<CampaignFoldersLinksItem>>,
    /// An array of objects representing campaign folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<CampaignFoldersFoldersItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl CampaignFolders {
    pub fn builder() -> CampaignFoldersBuilder {
        <CampaignFoldersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignFoldersBuilder {
    links: Option<Vec<CampaignFoldersLinksItem>>,
    folders: Option<Vec<CampaignFoldersFoldersItem>>,
    total_items: Option<i64>,
}

impl CampaignFoldersBuilder {
    pub fn links(mut self, value: Vec<CampaignFoldersLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn folders(mut self, value: Vec<CampaignFoldersFoldersItem>) -> Self {
        self.folders = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignFolders`].
    pub fn build(self) -> Result<CampaignFolders, BuildError> {
        Ok(CampaignFolders {
            links: self.links,
            folders: self.folders,
            total_items: self.total_items,
        })
    }
}
