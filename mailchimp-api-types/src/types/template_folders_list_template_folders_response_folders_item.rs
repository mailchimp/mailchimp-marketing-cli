pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A folder used to organize templates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTemplateFoldersResponseFoldersItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListTemplateFoldersResponseFoldersItemLinksItem>>,
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

impl ListTemplateFoldersResponseFoldersItem {
    pub fn builder() -> ListTemplateFoldersResponseFoldersItemBuilder {
        <ListTemplateFoldersResponseFoldersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTemplateFoldersResponseFoldersItemBuilder {
    links: Option<Vec<ListTemplateFoldersResponseFoldersItemLinksItem>>,
    count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
}

impl ListTemplateFoldersResponseFoldersItemBuilder {
    pub fn links(mut self, value: Vec<ListTemplateFoldersResponseFoldersItemLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListTemplateFoldersResponseFoldersItem`].
    pub fn build(self) -> Result<ListTemplateFoldersResponseFoldersItem, BuildError> {
        Ok(ListTemplateFoldersResponseFoldersItem {
            links: self.links,
            count: self.count,
            id: self.id,
            name: self.name,
        })
    }
}
