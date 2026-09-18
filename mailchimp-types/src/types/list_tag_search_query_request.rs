pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-tag-search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListTagSearchQueryRequest {
    /// The search query used to filter tags.  The search query will be compared to each tag as a prefix, so all tags that have a name starting with this field will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ListTagSearchQueryRequest {
    pub fn builder() -> ListTagSearchQueryRequestBuilder {
        <ListTagSearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListTagSearchQueryRequestBuilder {
    name: Option<String>,
}

impl ListTagSearchQueryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListTagSearchQueryRequest`].
    pub fn build(self) -> Result<ListTagSearchQueryRequest, BuildError> {
        Ok(ListTagSearchQueryRequest {
            name: self.name,
        })
    }
}

