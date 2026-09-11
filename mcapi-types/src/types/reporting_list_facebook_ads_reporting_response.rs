pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of Facebook ads.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFacebookAdsReportingResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFacebookAdsReportingResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_ads: Option<Vec<ReportingFacebookAd>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFacebookAdsReportingResponse {
    pub fn builder() -> ListFacebookAdsReportingResponseBuilder {
        <ListFacebookAdsReportingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFacebookAdsReportingResponseBuilder {
    links: Option<Vec<ListFacebookAdsReportingResponseLinksItem>>,
    facebook_ads: Option<Vec<ReportingFacebookAd>>,
    total_items: Option<i64>,
}

impl ListFacebookAdsReportingResponseBuilder {
    pub fn links(mut self, value: Vec<ListFacebookAdsReportingResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn facebook_ads(mut self, value: Vec<ReportingFacebookAd>) -> Self {
        self.facebook_ads = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFacebookAdsReportingResponse`].
    pub fn build(self) -> Result<ListFacebookAdsReportingResponse, BuildError> {
        Ok(ListFacebookAdsReportingResponse {
            links: self.links,
            facebook_ads: self.facebook_ads,
            total_items: self.total_items,
        })
    }
}
