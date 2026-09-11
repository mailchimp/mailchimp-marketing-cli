pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LandingPagesListQueryRequest {
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<ListLandingPagesRequestSortDir>,
    /// Returns files sorted by the specified field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListLandingPagesRequestSortField>,
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

impl LandingPagesListQueryRequest {
    pub fn builder() -> LandingPagesListQueryRequestBuilder {
        <LandingPagesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPagesListQueryRequestBuilder {
    sort_dir: Option<ListLandingPagesRequestSortDir>,
    sort_field: Option<ListLandingPagesRequestSortField>,
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
}

impl LandingPagesListQueryRequestBuilder {
    pub fn sort_dir(mut self, value: ListLandingPagesRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    pub fn sort_field(mut self, value: ListLandingPagesRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPagesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](LandingPagesListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](LandingPagesListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<LandingPagesListQueryRequest, BuildError> {
        Ok(LandingPagesListQueryRequest {
            sort_dir: self.sort_dir,
            sort_field: self.sort_field,
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
        })
    }
}

