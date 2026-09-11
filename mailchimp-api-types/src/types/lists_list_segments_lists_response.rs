pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of available segments.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSegmentsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSegmentsListsResponseLinksItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// An array of objects, each representing a list segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<List>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSegmentsListsResponse {
    pub fn builder() -> ListSegmentsListsResponseBuilder {
        <ListSegmentsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSegmentsListsResponseBuilder {
    links: Option<Vec<ListSegmentsListsResponseLinksItem>>,
    list_id: Option<String>,
    segments: Option<Vec<List>>,
    total_items: Option<i64>,
}

impl ListSegmentsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSegmentsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn segments(mut self, value: Vec<List>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSegmentsListsResponse`].
    pub fn build(self) -> Result<ListSegmentsListsResponse, BuildError> {
        Ok(ListSegmentsListsResponse {
            links: self.links,
            list_id: self.list_id,
            segments: self.segments,
            total_items: self.total_items,
        })
    }
}
