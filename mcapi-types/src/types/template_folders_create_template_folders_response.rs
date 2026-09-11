pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A folder used to organize templates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateTemplateFoldersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<CreateTemplateFoldersResponseLinksItem>>,
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

impl CreateTemplateFoldersResponse {
    pub fn builder() -> CreateTemplateFoldersResponseBuilder {
        <CreateTemplateFoldersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTemplateFoldersResponseBuilder {
    links: Option<Vec<CreateTemplateFoldersResponseLinksItem>>,
    count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
}

impl CreateTemplateFoldersResponseBuilder {
    pub fn links(mut self, value: Vec<CreateTemplateFoldersResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`CreateTemplateFoldersResponse`].
    pub fn build(self) -> Result<CreateTemplateFoldersResponse, BuildError> {
        Ok(CreateTemplateFoldersResponse {
            links: self.links,
            count: self.count,
            id: self.id,
            name: self.name,
        })
    }
}
