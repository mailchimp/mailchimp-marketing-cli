pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The HTML content for a landing page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListContentLandingPagesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListContentLandingPagesResponseLinksItem>>,
    /// The raw HTML for the landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// The JSON Structure for the landing page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<String>,
}

impl ListContentLandingPagesResponse {
    pub fn builder() -> ListContentLandingPagesResponseBuilder {
        <ListContentLandingPagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListContentLandingPagesResponseBuilder {
    links: Option<Vec<ListContentLandingPagesResponseLinksItem>>,
    html: Option<String>,
    json: Option<String>,
}

impl ListContentLandingPagesResponseBuilder {
    pub fn links(mut self, value: Vec<ListContentLandingPagesResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListContentLandingPagesResponse`].
    pub fn build(self) -> Result<ListContentLandingPagesResponse, BuildError> {
        Ok(ListContentLandingPagesResponse {
            links: self.links,
            html: self.html,
            json: self.json,
        })
    }
}
