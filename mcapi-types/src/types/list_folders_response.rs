pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of all folders in the File Manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFoldersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFoldersResponseLinksItem>>,
    /// A list of all folders in the File Manager.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<ListFoldersResponseFoldersItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFoldersResponse {
    pub fn builder() -> ListFoldersResponseBuilder {
        <ListFoldersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFoldersResponseBuilder {
    links: Option<Vec<ListFoldersResponseLinksItem>>,
    folders: Option<Vec<ListFoldersResponseFoldersItem>>,
    total_items: Option<i64>,
}

impl ListFoldersResponseBuilder {
    pub fn links(mut self, value: Vec<ListFoldersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn folders(mut self, value: Vec<ListFoldersResponseFoldersItem>) -> Self {
        self.folders = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFoldersResponse`].
    pub fn build(self) -> Result<ListFoldersResponse, BuildError> {
        Ok(ListFoldersResponse {
            links: self.links,
            folders: self.folders,
            total_items: self.total_items,
        })
    }
}
