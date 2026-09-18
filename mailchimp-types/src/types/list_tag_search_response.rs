pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of tags matching the input query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTagSearchResponse {
    /// A list of matching tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListTagSearchResponseTagsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListTagSearchResponse {
    pub fn builder() -> ListTagSearchResponseBuilder {
        <ListTagSearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTagSearchResponseBuilder {
    tags: Option<Vec<ListTagSearchResponseTagsItem>>,
    total_items: Option<i64>,
}

impl ListTagSearchResponseBuilder {
    pub fn tags(mut self, value: Vec<ListTagSearchResponseTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTagSearchResponse`].
    pub fn build(self) -> Result<ListTagSearchResponse, BuildError> {
        Ok(ListTagSearchResponse {
            tags: self.tags,
            total_items: self.total_items,
        })
    }
}
