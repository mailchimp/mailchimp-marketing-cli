pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// View members in a specific list segment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSegmentMembersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSegmentMembersResponseLinksItem>>,
    /// An array of objects, each representing a specific list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ListsSegmentsMembers>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSegmentMembersResponse {
    pub fn builder() -> ListSegmentMembersResponseBuilder {
        <ListSegmentMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSegmentMembersResponseBuilder {
    links: Option<Vec<ListSegmentMembersResponseLinksItem>>,
    members: Option<Vec<ListsSegmentsMembers>>,
    total_items: Option<i64>,
}

impl ListSegmentMembersResponseBuilder {
    pub fn links(mut self, value: Vec<ListSegmentMembersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn members(mut self, value: Vec<ListsSegmentsMembers>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSegmentMembersResponse`].
    pub fn build(self) -> Result<ListSegmentMembersResponse, BuildError> {
        Ok(ListSegmentMembersResponse {
            links: self.links,
            members: self.members,
            total_items: self.total_items,
        })
    }
}
