pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of member's subscriber activity in a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEmailActivityReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListEmailActivityReportsResponseLinksItem>>,
    /// The unique id for the sent campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// An array of members that were sent the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<EmailActivity>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListEmailActivityReportsResponse {
    pub fn builder() -> ListEmailActivityReportsResponseBuilder {
        <ListEmailActivityReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEmailActivityReportsResponseBuilder {
    links: Option<Vec<ListEmailActivityReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    emails: Option<Vec<EmailActivity>>,
    total_items: Option<i64>,
}

impl ListEmailActivityReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListEmailActivityReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn emails(mut self, value: Vec<EmailActivity>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEmailActivityReportsResponse`].
    pub fn build(self) -> Result<ListEmailActivityReportsResponse, BuildError> {
        Ok(ListEmailActivityReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            emails: self.emails,
            total_items: self.total_items,
        })
    }
}
