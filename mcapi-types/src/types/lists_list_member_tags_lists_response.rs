pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of tags assigned to a list member.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberTagsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberTagsListsResponseLinksItem>>,
    /// A list of tags assigned to the list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ListMemberTagsListsResponseTagsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberTagsListsResponse {
    pub fn builder() -> ListMemberTagsListsResponseBuilder {
        <ListMemberTagsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberTagsListsResponseBuilder {
    links: Option<Vec<ListMemberTagsListsResponseLinksItem>>,
    tags: Option<Vec<ListMemberTagsListsResponseTagsItem>>,
    total_items: Option<i64>,
}

impl ListMemberTagsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberTagsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<ListMemberTagsListsResponseTagsItem>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberTagsListsResponse`].
    pub fn build(self) -> Result<ListMemberTagsListsResponse, BuildError> {
        Ok(ListMemberTagsListsResponse {
            links: self.links,
            tags: self.tags,
            total_items: self.total_items,
        })
    }
}
