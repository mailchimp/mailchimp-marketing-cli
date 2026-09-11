pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Default content for a template.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDefaultContentTemplatesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListDefaultContentTemplatesResponseLinksItem>>,
    /// The sections that you can edit in the template, including each section's default content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sections: Option<HashMap<String, serde_json::Value>>,
}

impl ListDefaultContentTemplatesResponse {
    pub fn builder() -> ListDefaultContentTemplatesResponseBuilder {
        <ListDefaultContentTemplatesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDefaultContentTemplatesResponseBuilder {
    links: Option<Vec<ListDefaultContentTemplatesResponseLinksItem>>,
    sections: Option<HashMap<String, serde_json::Value>>,
}

impl ListDefaultContentTemplatesResponseBuilder {
    pub fn links(mut self, value: Vec<ListDefaultContentTemplatesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn sections(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.sections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDefaultContentTemplatesResponse`].
    pub fn build(self) -> Result<ListDefaultContentTemplatesResponse, BuildError> {
        Ok(ListDefaultContentTemplatesResponse {
            links: self.links,
            sections: self.sections,
        })
    }
}
