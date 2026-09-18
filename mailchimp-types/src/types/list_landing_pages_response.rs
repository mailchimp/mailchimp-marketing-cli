pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of landing pages.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListLandingPagesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListLandingPagesResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_pages: Option<Vec<LandingPageReport>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListLandingPagesResponse {
    pub fn builder() -> ListLandingPagesResponseBuilder {
        <ListLandingPagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLandingPagesResponseBuilder {
    links: Option<Vec<ListLandingPagesResponseLinksItem>>,
    landing_pages: Option<Vec<LandingPageReport>>,
    total_items: Option<i64>,
}

impl ListLandingPagesResponseBuilder {
    pub fn links(mut self, value: Vec<ListLandingPagesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn landing_pages(mut self, value: Vec<LandingPageReport>) -> Self {
        self.landing_pages = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLandingPagesResponse`].
    pub fn build(self) -> Result<ListLandingPagesResponse, BuildError> {
        Ok(ListLandingPagesResponse {
            links: self.links,
            landing_pages: self.landing_pages,
            total_items: self.total_items,
        })
    }
}
