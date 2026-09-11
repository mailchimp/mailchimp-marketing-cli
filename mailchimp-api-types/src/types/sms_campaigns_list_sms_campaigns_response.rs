pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of SMS campaigns.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSmsCampaignsResponse {
    /// An array of SMS campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_campaigns: Option<Vec<SmsCampaign>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSmsCampaignsResponseLinksItem>>,
}

impl ListSmsCampaignsResponse {
    pub fn builder() -> ListSmsCampaignsResponseBuilder {
        <ListSmsCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSmsCampaignsResponseBuilder {
    sms_campaigns: Option<Vec<SmsCampaign>>,
    total_items: Option<i64>,
    links: Option<Vec<ListSmsCampaignsResponseLinksItem>>,
}

impl ListSmsCampaignsResponseBuilder {
    pub fn sms_campaigns(mut self, value: Vec<SmsCampaign>) -> Self {
        self.sms_campaigns = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<ListSmsCampaignsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSmsCampaignsResponse`].
    pub fn build(self) -> Result<ListSmsCampaignsResponse, BuildError> {
        Ok(ListSmsCampaignsResponse {
            sms_campaigns: self.sms_campaigns,
            total_items: self.total_items,
            links: self.links,
        })
    }
}
