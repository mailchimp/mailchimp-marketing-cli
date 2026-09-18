pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of feedback based on a campaign's statistics.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAdviceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAdviceResponseLinksItem>>,
    /// An array of objects, each representing a point of campaign feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice: Option<Vec<ListAdviceResponseAdviceItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListAdviceResponse {
    pub fn builder() -> ListAdviceResponseBuilder {
        <ListAdviceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdviceResponseBuilder {
    links: Option<Vec<ListAdviceResponseLinksItem>>,
    advice: Option<Vec<ListAdviceResponseAdviceItem>>,
    campaign_id: Option<String>,
    total_items: Option<i64>,
}

impl ListAdviceResponseBuilder {
    pub fn links(mut self, value: Vec<ListAdviceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn advice(mut self, value: Vec<ListAdviceResponseAdviceItem>) -> Self {
        self.advice = Some(value);
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

    /// Consumes the builder and constructs a [`ListAdviceResponse`].
    pub fn build(self) -> Result<ListAdviceResponse, BuildError> {
        Ok(ListAdviceResponse {
            links: self.links,
            advice: self.advice,
            campaign_id: self.campaign_id,
            total_items: self.total_items,
        })
    }
}
