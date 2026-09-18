pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of tags assigned to a list member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberTagsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberTagsResponseLinksItem>>,
    /// A list of tags assigned to the list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListMemberTagsResponseTagsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberTagsResponse {
    pub fn builder() -> ListMemberTagsResponseBuilder {
        <ListMemberTagsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberTagsResponseBuilder {
    links: Option<Vec<ListMemberTagsResponseLinksItem>>,
    tags: Option<Vec<ListMemberTagsResponseTagsItem>>,
    total_items: Option<i64>,
}

impl ListMemberTagsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberTagsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<ListMemberTagsResponseTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberTagsResponse`].
    pub fn build(self) -> Result<ListMemberTagsResponse, BuildError> {
        Ok(ListMemberTagsResponse {
            links: self.links,
            tags: self.tags,
            total_items: self.total_items,
        })
    }
}
