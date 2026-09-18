pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Batch add/remove List members to/from static segment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchAddOrRemoveMembersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BatchAddOrRemoveMembersResponseLinksItem>>,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    /// An array of objects, each representing an array of email addresses that could not be added to the segment or removed and an error message providing more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchAddOrRemoveMembersResponseErrorsItem>>,
    /// An array of objects, each representing a new member that was added to the static segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_added: Option<Vec<ListsPost>>,
    /// An array of objects, each representing an existing list member that got deleted from the static segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<Vec<ListsPost>>,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_added: Option<i64>,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_removed: Option<i64>,
}

impl BatchAddOrRemoveMembersResponse {
    pub fn builder() -> BatchAddOrRemoveMembersResponseBuilder {
        <BatchAddOrRemoveMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchAddOrRemoveMembersResponseBuilder {
    links: Option<Vec<BatchAddOrRemoveMembersResponseLinksItem>>,
    error_count: Option<i64>,
    errors: Option<Vec<BatchAddOrRemoveMembersResponseErrorsItem>>,
    members_added: Option<Vec<ListsPost>>,
    members_removed: Option<Vec<ListsPost>>,
    total_added: Option<i64>,
    total_removed: Option<i64>,
}

impl BatchAddOrRemoveMembersResponseBuilder {
    pub fn links(mut self, value: Vec<BatchAddOrRemoveMembersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn error_count(mut self, value: i64) -> Self {
        self.error_count = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<BatchAddOrRemoveMembersResponseErrorsItem>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn members_added(mut self, value: Vec<ListsPost>) -> Self {
        self.members_added = Some(value);
        self
    }

    pub fn members_removed(mut self, value: Vec<ListsPost>) -> Self {
        self.members_removed = Some(value);
        self
    }

    pub fn total_added(mut self, value: i64) -> Self {
        self.total_added = Some(value);
        self
    }

    pub fn total_removed(mut self, value: i64) -> Self {
        self.total_removed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchAddOrRemoveMembersResponse`].
    pub fn build(self) -> Result<BatchAddOrRemoveMembersResponse, BuildError> {
        Ok(BatchAddOrRemoveMembersResponse {
            links: self.links,
            error_count: self.error_count,
            errors: self.errors,
            members_added: self.members_added,
            members_removed: self.members_removed,
            total_added: self.total_added,
            total_removed: self.total_removed,
        })
    }
}
