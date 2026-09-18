pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Up to the previous 180 days of daily detailed aggregated activity stats for a specific list. Does not include AutoResponder or Automation activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListActivityResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListActivityResponseLinksItem>>,
    /// Recent list activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Vec<ListActivityResponseActivityItem>>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListActivityResponse {
    pub fn builder() -> ListActivityResponseBuilder {
        <ListActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListActivityResponseBuilder {
    links: Option<Vec<ListActivityResponseLinksItem>>,
    activity: Option<Vec<ListActivityResponseActivityItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListActivityResponseBuilder {
    pub fn links(mut self, value: Vec<ListActivityResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn activity(mut self, value: Vec<ListActivityResponseActivityItem>) -> Self {
        self.activity = Some(value);
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

    /// Consumes the builder and constructs a [`ListActivityResponse`].
    pub fn build(self) -> Result<ListActivityResponse, BuildError> {
        Ok(ListActivityResponse {
            links: self.links,
            activity: self.activity,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
