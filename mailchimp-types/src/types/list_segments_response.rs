pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of available segments.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSegmentsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSegmentsResponseLinksItem>>,
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

impl ListSegmentsResponse {
    pub fn builder() -> ListSegmentsResponseBuilder {
        <ListSegmentsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSegmentsResponseBuilder {
    links: Option<Vec<ListSegmentsResponseLinksItem>>,
    list_id: Option<String>,
    segments: Option<Vec<List>>,
    total_items: Option<i64>,
}

impl ListSegmentsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSegmentsResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListSegmentsResponse`].
    pub fn build(self) -> Result<ListSegmentsResponse, BuildError> {
        Ok(ListSegmentsResponse {
            links: self.links,
            list_id: self.list_id,
            segments: self.segments,
            total_items: self.total_items,
        })
    }
}
