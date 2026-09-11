pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of batch requests that have been made.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBatchesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListBatchesResponseLinksItem>>,
    /// An array of objects representing batch calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batches: Option<Vec<Batch>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListBatchesResponse {
    pub fn builder() -> ListBatchesResponseBuilder {
        <ListBatchesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBatchesResponseBuilder {
    links: Option<Vec<ListBatchesResponseLinksItem>>,
    batches: Option<Vec<Batch>>,
    total_items: Option<i64>,
}

impl ListBatchesResponseBuilder {
    pub fn links(mut self, value: Vec<ListBatchesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn batches(mut self, value: Vec<Batch>) -> Self {
        self.batches = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListBatchesResponse`].
    pub fn build(self) -> Result<ListBatchesResponse, BuildError> {
        Ok(ListBatchesResponse {
            links: self.links,
            batches: self.batches,
            total_items: self.total_items,
        })
    }
}
