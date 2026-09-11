pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-segment
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetSegmentQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// Include cleaned members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cleaned: Option<bool>,
    /// Include transactional members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transactional: Option<bool>,
    /// Include unsubscribed members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unsubscribed: Option<bool>,
}

impl GetSegmentQueryRequest {
    pub fn builder() -> GetSegmentQueryRequestBuilder {
        <GetSegmentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSegmentQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    include_cleaned: Option<bool>,
    include_transactional: Option<bool>,
    include_unsubscribed: Option<bool>,
}

impl GetSegmentQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn include_cleaned(mut self, value: bool) -> Self {
        self.include_cleaned = Some(value);
        self
    }

    pub fn include_transactional(mut self, value: bool) -> Self {
        self.include_transactional = Some(value);
        self
    }

    pub fn include_unsubscribed(mut self, value: bool) -> Self {
        self.include_unsubscribed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSegmentQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](GetSegmentQueryRequestBuilder::fields)
    /// - [`exclude_fields`](GetSegmentQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<GetSegmentQueryRequest, BuildError> {
        Ok(GetSegmentQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            include_cleaned: self.include_cleaned,
            include_transactional: self.include_transactional,
            include_unsubscribed: self.include_unsubscribed,
        })
    }
}

