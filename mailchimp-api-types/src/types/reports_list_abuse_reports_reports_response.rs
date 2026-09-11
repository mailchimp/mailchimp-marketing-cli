pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of abuse complaints for a specific list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListAbuseReportsReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAbuseReportsReportsResponseLinksItem>>,
    /// An array of objects, each representing an abuse report resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abuse_reports: Option<Vec<AbuseComplaint>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListAbuseReportsReportsResponse {
    pub fn builder() -> ListAbuseReportsReportsResponseBuilder {
        <ListAbuseReportsReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAbuseReportsReportsResponseBuilder {
    links: Option<Vec<ListAbuseReportsReportsResponseLinksItem>>,
    abuse_reports: Option<Vec<AbuseComplaint>>,
    campaign_id: Option<String>,
    total_items: Option<i64>,
}

impl ListAbuseReportsReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListAbuseReportsReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn abuse_reports(mut self, value: Vec<AbuseComplaint>) -> Self {
        self.abuse_reports = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAbuseReportsReportsResponse`].
    pub fn build(self) -> Result<ListAbuseReportsReportsResponse, BuildError> {
        Ok(ListAbuseReportsReportsResponse {
            links: self.links,
            abuse_reports: self.abuse_reports,
            campaign_id: self.campaign_id,
            total_items: self.total_items,
        })
    }
}
