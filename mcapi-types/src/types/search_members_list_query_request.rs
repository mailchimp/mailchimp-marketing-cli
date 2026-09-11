pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchMembersListQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The search query used to filter results. Query should be a valid email, or a string representing a contact's first or last name.
    #[serde(default)]
    pub query: String,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
}

impl SearchMembersListQueryRequest {
    pub fn builder() -> SearchMembersListQueryRequestBuilder {
        <SearchMembersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchMembersListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    query: Option<String>,
    list_id: Option<String>,
}

impl SearchMembersListQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchMembersListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](SearchMembersListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](SearchMembersListQueryRequestBuilder::exclude_fields)
    /// - [`query`](SearchMembersListQueryRequestBuilder::query)
    pub fn build(self) -> Result<SearchMembersListQueryRequest, BuildError> {
        Ok(SearchMembersListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
            list_id: self.list_id,
        })
    }
}

