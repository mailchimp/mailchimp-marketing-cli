pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportsListQueryRequest {
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
    /// The campaign type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ListReportsRequestType>,
    /// Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub before_send_time: Option<DateTime<FixedOffset>>,
    /// Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_send_time: Option<DateTime<FixedOffset>>,
}

impl ReportsListQueryRequest {
    pub fn builder() -> ReportsListQueryRequestBuilder {
        <ReportsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    r#type: Option<ListReportsRequestType>,
    before_send_time: Option<DateTime<FixedOffset>>,
    since_send_time: Option<DateTime<FixedOffset>>,
}

impl ReportsListQueryRequestBuilder {
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

    pub fn r#type(mut self, value: ListReportsRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn before_send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before_send_time = Some(value);
        self
    }

    pub fn since_send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_send_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ReportsListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ReportsListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ReportsListQueryRequest, BuildError> {
        Ok(ReportsListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            r#type: self.r#type,
            before_send_time: self.before_send_time,
            since_send_time: self.since_send_time,
        })
    }
}

