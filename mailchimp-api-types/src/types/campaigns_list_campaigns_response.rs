pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An array of campaigns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCampaignsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListCampaignsResponseLinksItem>>,
    /// An array of campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaigns: Option<Vec<Campaigns>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListCampaignsResponse {
    pub fn builder() -> ListCampaignsResponseBuilder {
        <ListCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCampaignsResponseBuilder {
    links: Option<Vec<ListCampaignsResponseLinksItem>>,
    campaigns: Option<Vec<Campaigns>>,
    total_items: Option<i64>,
}

impl ListCampaignsResponseBuilder {
    pub fn links(mut self, value: Vec<ListCampaignsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaigns(mut self, value: Vec<Campaigns>) -> Self {
        self.campaigns = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCampaignsResponse`].
    pub fn build(self) -> Result<ListCampaignsResponse, BuildError> {
        Ok(ListCampaignsResponse {
            links: self.links,
            campaigns: self.campaigns,
            total_items: self.total_items,
        })
    }
}
