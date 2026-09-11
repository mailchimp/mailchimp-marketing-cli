pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Campaigns and Snippets found for given search term.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSearchCampaignsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSearchCampaignsResponseLinksItem>>,
    /// An array of matching campaigns and snippets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ListSearchCampaignsResponseResultsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSearchCampaignsResponse {
    pub fn builder() -> ListSearchCampaignsResponseBuilder {
        <ListSearchCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSearchCampaignsResponseBuilder {
    links: Option<Vec<ListSearchCampaignsResponseLinksItem>>,
    results: Option<Vec<ListSearchCampaignsResponseResultsItem>>,
    total_items: Option<i64>,
}

impl ListSearchCampaignsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSearchCampaignsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<ListSearchCampaignsResponseResultsItem>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSearchCampaignsResponse`].
    pub fn build(self) -> Result<ListSearchCampaignsResponse, BuildError> {
        Ok(ListSearchCampaignsResponse {
            links: self.links,
            results: self.results,
            total_items: self.total_items,
        })
    }
}
