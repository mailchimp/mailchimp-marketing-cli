pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of tags matching the input query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTagSearchListsResponse {
    /// A list of matching tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListTagSearchListsResponseTagsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListTagSearchListsResponse {
    pub fn builder() -> ListTagSearchListsResponseBuilder {
        <ListTagSearchListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTagSearchListsResponseBuilder {
    tags: Option<Vec<ListTagSearchListsResponseTagsItem>>,
    total_items: Option<i64>,
}

impl ListTagSearchListsResponseBuilder {
    pub fn tags(mut self, value: Vec<ListTagSearchListsResponseTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListTagSearchListsResponse`].
    pub fn build(self) -> Result<ListTagSearchListsResponse, BuildError> {
        Ok(ListTagSearchListsResponse {
            tags: self.tags,
            total_items: self.total_items,
        })
    }
}
