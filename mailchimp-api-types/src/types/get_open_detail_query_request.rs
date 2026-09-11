pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-open-detail
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetOpenDetailQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_bots: Option<bool>,
}

impl GetOpenDetailQueryRequest {
    pub fn builder() -> GetOpenDetailQueryRequestBuilder {
        <GetOpenDetailQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetOpenDetailQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    filter_bots: Option<bool>,
}

impl GetOpenDetailQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn filter_bots(mut self, value: bool) -> Self {
        self.filter_bots = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetOpenDetailQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](GetOpenDetailQueryRequestBuilder::fields)
    /// - [`exclude_fields`](GetOpenDetailQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<GetOpenDetailQueryRequest, BuildError> {
        Ok(GetOpenDetailQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            filter_bots: self.filter_bots,
        })
    }
}

