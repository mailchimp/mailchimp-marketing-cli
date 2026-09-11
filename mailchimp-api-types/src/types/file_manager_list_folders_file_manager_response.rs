pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of all folders in the File Manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFoldersFileManagerResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFoldersFileManagerResponseLinksItem>>,
    /// A list of all folders in the File Manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<ListFoldersFileManagerResponseFoldersItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFoldersFileManagerResponse {
    pub fn builder() -> ListFoldersFileManagerResponseBuilder {
        <ListFoldersFileManagerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFoldersFileManagerResponseBuilder {
    links: Option<Vec<ListFoldersFileManagerResponseLinksItem>>,
    folders: Option<Vec<ListFoldersFileManagerResponseFoldersItem>>,
    total_items: Option<i64>,
}

impl ListFoldersFileManagerResponseBuilder {
    pub fn links(mut self, value: Vec<ListFoldersFileManagerResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn folders(mut self, value: Vec<ListFoldersFileManagerResponseFoldersItem>) -> Self {
        self.folders = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFoldersFileManagerResponse`].
    pub fn build(self) -> Result<ListFoldersFileManagerResponse, BuildError> {
        Ok(ListFoldersFileManagerResponse {
            links: self.links,
            folders: self.folders,
            total_items: self.total_items,
        })
    }
}
