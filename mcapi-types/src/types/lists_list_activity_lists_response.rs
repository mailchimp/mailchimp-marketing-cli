pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Up to the previous 180 days of daily detailed aggregated activity stats for a specific list. Does not include AutoResponder or Automation activity.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListActivityListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListActivityListsResponseLinksItem>>,
    /// Recent list activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<Vec<ListActivityListsResponseActivityItem>>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListActivityListsResponse {
    pub fn builder() -> ListActivityListsResponseBuilder {
        <ListActivityListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListActivityListsResponseBuilder {
    links: Option<Vec<ListActivityListsResponseLinksItem>>,
    activity: Option<Vec<ListActivityListsResponseActivityItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListActivityListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListActivityListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn activity(mut self, value: Vec<ListActivityListsResponseActivityItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListActivityListsResponse`].
    pub fn build(self) -> Result<ListActivityListsResponse, BuildError> {
        Ok(ListActivityListsResponse {
            links: self.links,
            activity: self.activity,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
