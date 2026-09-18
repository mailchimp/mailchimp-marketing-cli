pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Batch update list members.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchSubscribeOrUnsubscribeResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<BatchSubscribeOrUnsubscribeResponseLinksItem>>,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    /// An array of objects, each representing an email address that could not be added to the list or updated and an error message providing more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<BatchSubscribeOrUnsubscribeResponseErrorsItem>>,
    /// An array of objects, each representing a new member that was added to the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_members: Option<Vec<ListsPost>>,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_created: Option<i64>,
    /// The total number of items matching the query, irrespective of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_updated: Option<i64>,
    /// An array of objects, each representing an existing list member whose subscription status was updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_members: Option<Vec<ListsPost>>,
}

impl BatchSubscribeOrUnsubscribeResponse {
    pub fn builder() -> BatchSubscribeOrUnsubscribeResponseBuilder {
        <BatchSubscribeOrUnsubscribeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchSubscribeOrUnsubscribeResponseBuilder {
    links: Option<Vec<BatchSubscribeOrUnsubscribeResponseLinksItem>>,
    error_count: Option<i64>,
    errors: Option<Vec<BatchSubscribeOrUnsubscribeResponseErrorsItem>>,
    new_members: Option<Vec<ListsPost>>,
    total_created: Option<i64>,
    total_updated: Option<i64>,
    updated_members: Option<Vec<ListsPost>>,
}

impl BatchSubscribeOrUnsubscribeResponseBuilder {
    pub fn links(mut self, value: Vec<BatchSubscribeOrUnsubscribeResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn error_count(mut self, value: i64) -> Self {
        self.error_count = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<BatchSubscribeOrUnsubscribeResponseErrorsItem>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn new_members(mut self, value: Vec<ListsPost>) -> Self {
        self.new_members = Some(value);
        self
    }

    pub fn total_created(mut self, value: i64) -> Self {
        self.total_created = Some(value);
        self
    }

    pub fn total_updated(mut self, value: i64) -> Self {
        self.total_updated = Some(value);
        self
    }

    pub fn updated_members(mut self, value: Vec<ListsPost>) -> Self {
        self.updated_members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchSubscribeOrUnsubscribeResponse`].
    pub fn build(self) -> Result<BatchSubscribeOrUnsubscribeResponse, BuildError> {
        Ok(BatchSubscribeOrUnsubscribeResponse {
            links: self.links,
            error_count: self.error_count,
            errors: self.errors,
            new_members: self.new_members,
            total_created: self.total_created,
            total_updated: self.total_updated,
            updated_members: self.updated_members,
        })
    }
}
