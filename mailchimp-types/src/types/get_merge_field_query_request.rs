pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-merge-field
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetMergeFieldQueryRequest {
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
}

impl GetMergeFieldQueryRequest {
    pub fn builder() -> GetMergeFieldQueryRequestBuilder {
        <GetMergeFieldQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMergeFieldQueryRequestBuilder {
    exclude_fields: Option<Vec<Option<String>>>,
    fields: Option<Vec<Option<String>>>,
}

impl GetMergeFieldQueryRequestBuilder {
    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetMergeFieldQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`exclude_fields`](GetMergeFieldQueryRequestBuilder::exclude_fields)
    /// - [`fields`](GetMergeFieldQueryRequestBuilder::fields)
    pub fn build(self) -> Result<GetMergeFieldQueryRequest, BuildError> {
        Ok(GetMergeFieldQueryRequest {
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}

