pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A folder used to organize templates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListResponseFoldersItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListResponseFoldersItemLinksItem>>,
    /// The number of templates in the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// A string that uniquely identifies this template folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the folder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListResponseFoldersItem {
    pub fn builder() -> ListResponseFoldersItemBuilder {
        <ListResponseFoldersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseFoldersItemBuilder {
    links: Option<Vec<ListResponseFoldersItemLinksItem>>,
    count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
}

impl ListResponseFoldersItemBuilder {
    pub fn links(mut self, value: Vec<ListResponseFoldersItemLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListResponseFoldersItem`].
    pub fn build(self) -> Result<ListResponseFoldersItem, BuildError> {
        Ok(ListResponseFoldersItem {
            links: self.links,
            count: self.count,
            id: self.id,
            name: self.name,
        })
    }
}
