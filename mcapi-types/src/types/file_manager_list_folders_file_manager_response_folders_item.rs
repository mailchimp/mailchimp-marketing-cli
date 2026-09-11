pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An individual folder listed in the File Manager.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFoldersFileManagerResponseFoldersItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFoldersFileManagerResponseFoldersItemLinksItem>>,
    /// The date and time a file was added to the File Manager in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The username of the profile that created the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The number of files in the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i64>,
    /// The unique id for the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListFoldersFileManagerResponseFoldersItem {
    pub fn builder() -> ListFoldersFileManagerResponseFoldersItemBuilder {
        <ListFoldersFileManagerResponseFoldersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFoldersFileManagerResponseFoldersItemBuilder {
    links: Option<Vec<ListFoldersFileManagerResponseFoldersItemLinksItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    file_count: Option<i64>,
    id: Option<i64>,
    name: Option<String>,
}

impl ListFoldersFileManagerResponseFoldersItemBuilder {
    pub fn links(mut self, value: Vec<ListFoldersFileManagerResponseFoldersItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn file_count(mut self, value: i64) -> Self {
        self.file_count = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListFoldersFileManagerResponseFoldersItem`].
    pub fn build(self) -> Result<ListFoldersFileManagerResponseFoldersItem, BuildError> {
        Ok(ListFoldersFileManagerResponseFoldersItem {
            links: self.links,
            created_at: self.created_at,
            created_by: self.created_by,
            file_count: self.file_count,
            id: self.id,
            name: self.name,
        })
    }
}
