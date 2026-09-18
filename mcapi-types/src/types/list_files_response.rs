pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of available images and files stored in the File Manager for the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFilesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFilesResponseLinksItem>>,
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

impl ListFilesResponse {
    pub fn builder() -> ListFilesResponseBuilder {
        <ListFilesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFilesResponseBuilder {
    links: Option<Vec<ListFilesResponseLinksItem>>,
    files: Option<Vec<GalleryFile>>,
    total_file_size: Option<f64>,
    total_items: Option<i64>,
}

impl ListFilesResponseBuilder {
    pub fn links(mut self, value: Vec<ListFilesResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListFilesResponse`].
    pub fn build(self) -> Result<ListFilesResponse, BuildError> {
        Ok(ListFilesResponse {
            links: self.links,
            files: self.files,
            total_file_size: self.total_file_size,
            total_items: self.total_items,
        })
    }
}
