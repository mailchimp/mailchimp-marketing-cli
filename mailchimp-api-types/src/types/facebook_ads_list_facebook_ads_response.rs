pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Contains an array of facebook ads.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFacebookAdsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFacebookAdsResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_ads: Option<Vec<FacebookAds>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFacebookAdsResponse {
    pub fn builder() -> ListFacebookAdsResponseBuilder {
        <ListFacebookAdsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFacebookAdsResponseBuilder {
    links: Option<Vec<ListFacebookAdsResponseLinksItem>>,
    facebook_ads: Option<Vec<FacebookAds>>,
    total_items: Option<i64>,
}

impl ListFacebookAdsResponseBuilder {
    pub fn links(mut self, value: Vec<ListFacebookAdsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn facebook_ads(mut self, value: Vec<FacebookAds>) -> Self {
        self.facebook_ads = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFacebookAdsResponse`].
    pub fn build(self) -> Result<ListFacebookAdsResponse, BuildError> {
        Ok(ListFacebookAdsResponse {
            links: self.links,
            facebook_ads: self.facebook_ads,
            total_items: self.total_items,
        })
    }
}
