pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Exact matches of the provided search query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListResponseExactMatches {
    /// An array of objects, each representing a specific list member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ListMembers>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListResponseExactMatches {
    pub fn builder() -> ListResponseExactMatchesBuilder {
        <ListResponseExactMatchesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseExactMatchesBuilder {
    members: Option<Vec<ListMembers>>,
    total_items: Option<i64>,
}

impl ListResponseExactMatchesBuilder {
    pub fn members(mut self, value: Vec<ListMembers>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponseExactMatches`].
    pub fn build(self) -> Result<ListResponseExactMatches, BuildError> {
        Ok(ListResponseExactMatches {
            members: self.members,
            total_items: self.total_items,
        })
    }
}
