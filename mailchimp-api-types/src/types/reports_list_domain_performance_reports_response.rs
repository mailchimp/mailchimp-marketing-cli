pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Statistics for the top-performing email domains in a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDomainPerformanceReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListDomainPerformanceReportsResponseLinksItem>>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The top 5 email domains based on total delivered emails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<ListDomainPerformanceReportsResponseDomainsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// The total number of emails sent for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_sent: Option<i64>,
}

impl ListDomainPerformanceReportsResponse {
    pub fn builder() -> ListDomainPerformanceReportsResponseBuilder {
        <ListDomainPerformanceReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDomainPerformanceReportsResponseBuilder {
    links: Option<Vec<ListDomainPerformanceReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    domains: Option<Vec<ListDomainPerformanceReportsResponseDomainsItem>>,
    total_items: Option<i64>,
    total_sent: Option<i64>,
}

impl ListDomainPerformanceReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListDomainPerformanceReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn domains(mut self, value: Vec<ListDomainPerformanceReportsResponseDomainsItem>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn total_sent(mut self, value: i64) -> Self {
        self.total_sent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDomainPerformanceReportsResponse`].
    pub fn build(self) -> Result<ListDomainPerformanceReportsResponse, BuildError> {
        Ok(ListDomainPerformanceReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            domains: self.domains,
            total_items: self.total_items,
            total_sent: self.total_sent,
        })
    }
}
