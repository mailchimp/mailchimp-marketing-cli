pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of template folders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTemplateFoldersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListTemplateFoldersResponseLinksItem>>,
    /// An array of objects representing template folders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folders: Option<Vec<ListTemplateFoldersResponseFoldersItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListTemplateFoldersResponse {
    pub fn builder() -> ListTemplateFoldersResponseBuilder {
        <ListTemplateFoldersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTemplateFoldersResponseBuilder {
    links: Option<Vec<ListTemplateFoldersResponseLinksItem>>,
    folders: Option<Vec<ListTemplateFoldersResponseFoldersItem>>,
    total_items: Option<i64>,
}

impl ListTemplateFoldersResponseBuilder {
    pub fn links(mut self, value: Vec<ListTemplateFoldersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn folders(mut self, value: Vec<ListTemplateFoldersResponseFoldersItem>) -> Self {
        self.folders = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTemplateFoldersResponse`].
    pub fn build(self) -> Result<ListTemplateFoldersResponse, BuildError> {
        Ok(ListTemplateFoldersResponse {
            links: self.links,
            folders: self.folders,
            total_items: self.total_items,
        })
    }
}
