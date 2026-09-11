pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of social activity for the campaign, tracked by EepURL.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEepurlReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListEepurlReportsResponseLinksItem>>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// A summary of the click-throughs on the campaign's URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<ListEepurlReportsResponseClicks>,
    /// The shortened link used for tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eepurl: Option<String>,
    /// A summary of the top referrers for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrers: Option<Vec<ListEepurlReportsResponseReferrersItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// A summary of Twitter activity for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<ListEepurlReportsResponseTwitter>,
}

impl ListEepurlReportsResponse {
    pub fn builder() -> ListEepurlReportsResponseBuilder {
        <ListEepurlReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEepurlReportsResponseBuilder {
    links: Option<Vec<ListEepurlReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    clicks: Option<ListEepurlReportsResponseClicks>,
    eepurl: Option<String>,
    referrers: Option<Vec<ListEepurlReportsResponseReferrersItem>>,
    total_items: Option<i64>,
    twitter: Option<ListEepurlReportsResponseTwitter>,
}

impl ListEepurlReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListEepurlReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn clicks(mut self, value: ListEepurlReportsResponseClicks) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn eepurl(mut self, value: impl Into<String>) -> Self {
        self.eepurl = Some(value.into());
        self
    }

    pub fn referrers(mut self, value: Vec<ListEepurlReportsResponseReferrersItem>) -> Self {
        self.referrers = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn twitter(mut self, value: ListEepurlReportsResponseTwitter) -> Self {
        self.twitter = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEepurlReportsResponse`].
    pub fn build(self) -> Result<ListEepurlReportsResponse, BuildError> {
        Ok(ListEepurlReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            clicks: self.clicks,
            eepurl: self.eepurl,
            referrers: self.referrers,
            total_items: self.total_items,
            twitter: self.twitter,
        })
    }
}
