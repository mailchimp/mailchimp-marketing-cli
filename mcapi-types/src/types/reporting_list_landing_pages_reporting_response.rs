pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of landing pages.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListLandingPagesReportingResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListLandingPagesReportingResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landing_pages: Option<Vec<LandingPageReport>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListLandingPagesReportingResponse {
    pub fn builder() -> ListLandingPagesReportingResponseBuilder {
        <ListLandingPagesReportingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLandingPagesReportingResponseBuilder {
    links: Option<Vec<ListLandingPagesReportingResponseLinksItem>>,
    landing_pages: Option<Vec<LandingPageReport>>,
    total_items: Option<i64>,
}

impl ListLandingPagesReportingResponseBuilder {
    pub fn links(mut self, value: Vec<ListLandingPagesReportingResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListLandingPagesReportingResponse`].
    pub fn build(self) -> Result<ListLandingPagesReportingResponse, BuildError> {
        Ok(ListLandingPagesReportingResponse {
            links: self.links,
            landing_pages: self.landing_pages,
            total_items: self.total_items,
        })
    }
}
