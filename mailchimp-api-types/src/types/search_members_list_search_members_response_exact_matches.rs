pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Exact matches of the provided search query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSearchMembersResponseExactMatches {
    /// An array of objects, each representing a specific list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ListMembers>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListSearchMembersResponseExactMatches {
    pub fn builder() -> ListSearchMembersResponseExactMatchesBuilder {
        <ListSearchMembersResponseExactMatchesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSearchMembersResponseExactMatchesBuilder {
    members: Option<Vec<ListMembers>>,
    total_items: Option<i64>,
}

impl ListSearchMembersResponseExactMatchesBuilder {
    pub fn members(mut self, value: Vec<ListMembers>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSearchMembersResponseExactMatches`].
    pub fn build(self) -> Result<ListSearchMembersResponseExactMatches, BuildError> {
        Ok(ListSearchMembersResponseExactMatches {
            members: self.members,
            total_items: self.total_items,
        })
    }
}
