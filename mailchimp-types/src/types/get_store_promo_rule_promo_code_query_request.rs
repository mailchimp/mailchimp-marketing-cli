pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-store-promo-rule-promo-code
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetStorePromoRulePromoCodeQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
}

impl GetStorePromoRulePromoCodeQueryRequest {
    pub fn builder() -> GetStorePromoRulePromoCodeQueryRequestBuilder {
        <GetStorePromoRulePromoCodeQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetStorePromoRulePromoCodeQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
}

impl GetStorePromoRulePromoCodeQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetStorePromoRulePromoCodeQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](GetStorePromoRulePromoCodeQueryRequestBuilder::fields)
    /// - [`exclude_fields`](GetStorePromoRulePromoCodeQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<GetStorePromoRulePromoCodeQueryRequest, BuildError> {
        Ok(GetStorePromoRulePromoCodeQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
        })
    }
}

