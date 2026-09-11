pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A detailed report of any campaign emails that were opened by a list member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOpenDetailsReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListOpenDetailsReportsResponseLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// An array of objects, each representing a list member who opened a campaign email. Each members object will contain information about the number of total opens by a single member, as well as timestamps for each open event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<OpenActivity>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// The total number of opens matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_opens: Option<i64>,
    /// The total number of opens excluding opens from email clients that use proxies regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_proxy_excluded_opens: Option<i64>,
}

impl ListOpenDetailsReportsResponse {
    pub fn builder() -> ListOpenDetailsReportsResponseBuilder {
        <ListOpenDetailsReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOpenDetailsReportsResponseBuilder {
    links: Option<Vec<ListOpenDetailsReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    members: Option<Vec<OpenActivity>>,
    total_items: Option<i64>,
    total_opens: Option<i64>,
    total_proxy_excluded_opens: Option<i64>,
}

impl ListOpenDetailsReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListOpenDetailsReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<OpenActivity>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn total_opens(mut self, value: i64) -> Self {
        self.total_opens = Some(value);
        self
    }

    pub fn total_proxy_excluded_opens(mut self, value: i64) -> Self {
        self.total_proxy_excluded_opens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOpenDetailsReportsResponse`].
    pub fn build(self) -> Result<ListOpenDetailsReportsResponse, BuildError> {
        Ok(ListOpenDetailsReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            members: self.members,
            total_items: self.total_items,
            total_opens: self.total_opens,
            total_proxy_excluded_opens: self.total_proxy_excluded_opens,
        })
    }
}
