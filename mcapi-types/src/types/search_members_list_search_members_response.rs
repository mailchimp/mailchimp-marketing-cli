pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Members found for given search term
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListSearchMembersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSearchMembersResponseLinksItem>>,
    /// Exact matches of the provided search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_matches: Option<ListSearchMembersResponseExactMatches>,
    /// Partial matches of the provided search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_search: Option<ListSearchMembersResponseFullSearch>,
}

impl ListSearchMembersResponse {
    pub fn builder() -> ListSearchMembersResponseBuilder {
        <ListSearchMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSearchMembersResponseBuilder {
    links: Option<Vec<ListSearchMembersResponseLinksItem>>,
    exact_matches: Option<ListSearchMembersResponseExactMatches>,
    full_search: Option<ListSearchMembersResponseFullSearch>,
}

impl ListSearchMembersResponseBuilder {
    pub fn links(mut self, value: Vec<ListSearchMembersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn exact_matches(mut self, value: ListSearchMembersResponseExactMatches) -> Self {
        self.exact_matches = Some(value);
        self
    }

    pub fn full_search(mut self, value: ListSearchMembersResponseFullSearch) -> Self {
        self.full_search = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSearchMembersResponse`].
    pub fn build(self) -> Result<ListSearchMembersResponse, BuildError> {
        Ok(ListSearchMembersResponse {
            links: self.links,
            exact_matches: self.exact_matches,
            full_search: self.full_search,
        })
    }
}
