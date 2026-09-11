pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of available images and files stored in the File Manager for the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFolderFilesFileManagerResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFolderFilesFileManagerResponseLinksItem>>,
    /// A list of files and images in an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<GalleryFile>>,
    /// The total size of all File Manager files in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_file_size: Option<f64>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFolderFilesFileManagerResponse {
    pub fn builder() -> ListFolderFilesFileManagerResponseBuilder {
        <ListFolderFilesFileManagerResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFolderFilesFileManagerResponseBuilder {
    links: Option<Vec<ListFolderFilesFileManagerResponseLinksItem>>,
    files: Option<Vec<GalleryFile>>,
    total_file_size: Option<f64>,
    total_items: Option<i64>,
}

impl ListFolderFilesFileManagerResponseBuilder {
    pub fn links(mut self, value: Vec<ListFolderFilesFileManagerResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<GalleryFile>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn total_file_size(mut self, value: f64) -> Self {
        self.total_file_size = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFolderFilesFileManagerResponse`].
    pub fn build(self) -> Result<ListFolderFilesFileManagerResponse, BuildError> {
        Ok(ListFolderFilesFileManagerResponse {
            links: self.links,
            files: self.files,
            total_file_size: self.total_file_size,
            total_items: self.total_items,
        })
    }
}
