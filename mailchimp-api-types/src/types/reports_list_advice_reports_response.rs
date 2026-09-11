pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of feedback based on a campaign's statistics.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAdviceReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAdviceReportsResponseLinksItem>>,
    /// An array of objects, each representing a point of campaign feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advice: Option<Vec<ListAdviceReportsResponseAdviceItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListAdviceReportsResponse {
    pub fn builder() -> ListAdviceReportsResponseBuilder {
        <ListAdviceReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAdviceReportsResponseBuilder {
    links: Option<Vec<ListAdviceReportsResponseLinksItem>>,
    advice: Option<Vec<ListAdviceReportsResponseAdviceItem>>,
    campaign_id: Option<String>,
    total_items: Option<i64>,
}

impl ListAdviceReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListAdviceReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn advice(mut self, value: Vec<ListAdviceReportsResponseAdviceItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListAdviceReportsResponse`].
    pub fn build(self) -> Result<ListAdviceReportsResponse, BuildError> {
        Ok(ListAdviceReportsResponse {
            links: self.links,
            advice: self.advice,
            campaign_id: self.campaign_id,
            total_items: self.total_items,
        })
    }
}
