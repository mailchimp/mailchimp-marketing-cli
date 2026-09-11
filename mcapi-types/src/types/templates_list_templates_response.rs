pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list an account's available templates.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTemplatesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListTemplatesResponseLinksItem>>,
    /// All of an account's saved or custom templates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub templates: Option<Vec<TemplateInstance>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListTemplatesResponse {
    pub fn builder() -> ListTemplatesResponseBuilder {
        <ListTemplatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTemplatesResponseBuilder {
    links: Option<Vec<ListTemplatesResponseLinksItem>>,
    templates: Option<Vec<TemplateInstance>>,
    total_items: Option<i64>,
}

impl ListTemplatesResponseBuilder {
    pub fn links(mut self, value: Vec<ListTemplatesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn templates(mut self, value: Vec<TemplateInstance>) -> Self {
        self.templates = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTemplatesResponse`].
    pub fn build(self) -> Result<ListTemplatesResponse, BuildError> {
        Ok(ListTemplatesResponse {
            links: self.links,
            templates: self.templates,
            total_items: self.total_items,
        })
    }
}
