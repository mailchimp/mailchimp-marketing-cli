pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchCampaignsListQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The search query used to filter results.
    #[serde(default)]
    pub query: String,
}

impl SearchCampaignsListQueryRequest {
    pub fn builder() -> SearchCampaignsListQueryRequestBuilder {
        <SearchCampaignsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchCampaignsListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    query: Option<String>,
}

impl SearchCampaignsListQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`SearchCampaignsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](SearchCampaignsListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](SearchCampaignsListQueryRequestBuilder::exclude_fields)
    /// - [`query`](SearchCampaignsListQueryRequestBuilder::query)
    pub fn build(self) -> Result<SearchCampaignsListQueryRequest, BuildError> {
        Ok(SearchCampaignsListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            query: self.query.ok_or_else(|| BuildError::missing_field("query"))?,
        })
    }
}

