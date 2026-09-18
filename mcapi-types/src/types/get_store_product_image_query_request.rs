pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-store-product-image
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetStoreProductImageQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
}

impl GetStoreProductImageQueryRequest {
    pub fn builder() -> GetStoreProductImageQueryRequestBuilder {
        <GetStoreProductImageQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStoreProductImageQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
}

impl GetStoreProductImageQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetStoreProductImageQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](GetStoreProductImageQueryRequestBuilder::fields)
    /// - [`exclude_fields`](GetStoreProductImageQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<GetStoreProductImageQueryRequest, BuildError> {
        Ok(GetStoreProductImageQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
        })
    }
}

