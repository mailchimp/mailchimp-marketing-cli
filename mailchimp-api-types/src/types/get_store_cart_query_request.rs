pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-store-cart
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetStoreCartQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
}

impl GetStoreCartQueryRequest {
    pub fn builder() -> GetStoreCartQueryRequestBuilder {
        <GetStoreCartQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStoreCartQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
}

impl GetStoreCartQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetStoreCartQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](GetStoreCartQueryRequestBuilder::fields)
    /// - [`exclude_fields`](GetStoreCartQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<GetStoreCartQueryRequest, BuildError> {
        Ok(GetStoreCartQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
        })
    }
}

