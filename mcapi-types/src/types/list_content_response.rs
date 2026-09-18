pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The HTML content for a landing page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListContentResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListContentResponseLinksItem>>,
    /// The raw HTML for the landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// The JSON Structure for the landing page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,
}

impl ListContentResponse {
    pub fn builder() -> ListContentResponseBuilder {
        <ListContentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListContentResponseBuilder {
    links: Option<Vec<ListContentResponseLinksItem>>,
    html: Option<String>,
    json: Option<String>,
}

impl ListContentResponseBuilder {
    pub fn links(mut self, value: Vec<ListContentResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn json(mut self, value: impl Into<String>) -> Self {
        self.json = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListContentResponse`].
    pub fn build(self) -> Result<ListContentResponse, BuildError> {
        Ok(ListContentResponse {
            links: self.links,
            html: self.html,
            json: self.json,
        })
    }
}
