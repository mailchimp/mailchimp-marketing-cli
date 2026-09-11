pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of reports containing campaigns marked as Sent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListReportsResponseLinksItem>>,
    /// An array of objects, each representing a report resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<CampaignReport>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListReportsResponse {
    pub fn builder() -> ListReportsResponseBuilder {
        <ListReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReportsResponseBuilder {
    links: Option<Vec<ListReportsResponseLinksItem>>,
    reports: Option<Vec<CampaignReport>>,
    total_items: Option<i64>,
}

impl ListReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn reports(mut self, value: Vec<CampaignReport>) -> Self {
        self.reports = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReportsResponse`].
    pub fn build(self) -> Result<ListReportsResponse, BuildError> {
        Ok(ListReportsResponse {
            links: self.links,
            reports: self.reports,
            total_items: self.total_items,
        })
    }
}
