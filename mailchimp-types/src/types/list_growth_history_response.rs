pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A month-by-month summary of a specific list's growth activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListGrowthHistoryResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListGrowthHistoryResponseLinksItem>>,
    /// An array of objects, each representing a monthly growth report for a list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<GrowthHistory>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListGrowthHistoryResponse {
    pub fn builder() -> ListGrowthHistoryResponseBuilder {
        <ListGrowthHistoryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListGrowthHistoryResponseBuilder {
    links: Option<Vec<ListGrowthHistoryResponseLinksItem>>,
    history: Option<Vec<GrowthHistory>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListGrowthHistoryResponseBuilder {
    pub fn links(mut self, value: Vec<ListGrowthHistoryResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn history(mut self, value: Vec<GrowthHistory>) -> Self {
        self.history = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListGrowthHistoryResponse`].
    pub fn build(self) -> Result<ListGrowthHistoryResponse, BuildError> {
        Ok(ListGrowthHistoryResponse {
            links: self.links,
            history: self.history,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
