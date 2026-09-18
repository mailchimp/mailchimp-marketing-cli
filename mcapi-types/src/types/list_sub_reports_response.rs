pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of reports containing child campaigns for a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSubReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSubReportsResponseLinksItem>>,
    /// Unique identifier of the parent campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// An array of objects, each representing a report resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<Vec<CampaignReport>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSubReportsResponse {
    pub fn builder() -> ListSubReportsResponseBuilder {
        <ListSubReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSubReportsResponseBuilder {
    links: Option<Vec<ListSubReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    reports: Option<Vec<CampaignReport>>,
    total_items: Option<i64>,
}

impl ListSubReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSubReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListSubReportsResponse`].
    pub fn build(self) -> Result<ListSubReportsResponse, BuildError> {
        Ok(ListSubReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            reports: self.reports,
            total_items: self.total_items,
        })
    }
}
