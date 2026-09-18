pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-segments
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSegmentsQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limit results based on segment type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Restrict results to segments created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_created_at: Option<String>,
    /// Restrict results to segments created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_created_at: Option<String>,
    /// Include cleaned members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cleaned: Option<bool>,
    /// Include transactional members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transactional: Option<bool>,
    /// Include unsubscribed members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unsubscribed: Option<bool>,
    /// Restrict results to segments update after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_updated_at: Option<String>,
    /// Restrict results to segments update before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_updated_at: Option<String>,
    /// Exclude results based on segment type. For example, use `exclude_type=static` to exclude tags from the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_type: Option<ListSegmentsRequestExcludeType>,
}

impl ListSegmentsQueryRequest {
    pub fn builder() -> ListSegmentsQueryRequestBuilder {
        <ListSegmentsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSegmentsQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    r#type: Option<String>,
    since_created_at: Option<String>,
    before_created_at: Option<String>,
    include_cleaned: Option<bool>,
    include_transactional: Option<bool>,
    include_unsubscribed: Option<bool>,
    since_updated_at: Option<String>,
    before_updated_at: Option<String>,
    exclude_type: Option<ListSegmentsRequestExcludeType>,
}

impl ListSegmentsQueryRequestBuilder {
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

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn since_created_at(mut self, value: impl Into<String>) -> Self {
        self.since_created_at = Some(value.into());
        self
    }

    pub fn before_created_at(mut self, value: impl Into<String>) -> Self {
        self.before_created_at = Some(value.into());
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

    pub fn since_updated_at(mut self, value: impl Into<String>) -> Self {
        self.since_updated_at = Some(value.into());
        self
    }

    pub fn before_updated_at(mut self, value: impl Into<String>) -> Self {
        self.before_updated_at = Some(value.into());
        self
    }

    pub fn exclude_type(mut self, value: ListSegmentsRequestExcludeType) -> Self {
        self.exclude_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSegmentsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListSegmentsQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListSegmentsQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListSegmentsQueryRequest, BuildError> {
        Ok(ListSegmentsQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            r#type: self.r#type,
            since_created_at: self.since_created_at,
            before_created_at: self.before_created_at,
            include_cleaned: self.include_cleaned,
            include_transactional: self.include_transactional,
            include_unsubscribed: self.include_unsubscribed,
            since_updated_at: self.since_updated_at,
            before_updated_at: self.before_updated_at,
            exclude_type: self.exclude_type,
        })
    }
}

