pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationsListQueryRequest {
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// Restrict the response to automations created before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub before_create_time: Option<DateTime<FixedOffset>>,
    /// Restrict the response to automations created after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_create_time: Option<DateTime<FixedOffset>>,
    /// Restrict the response to automations started before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub before_start_time: Option<DateTime<FixedOffset>>,
    /// Restrict the response to automations started after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_start_time: Option<DateTime<FixedOffset>>,
    /// Restrict the results to automations with the specified status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAutomationsRequestStatus>,
}

impl AutomationsListQueryRequest {
    pub fn builder() -> AutomationsListQueryRequestBuilder {
        <AutomationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationsListQueryRequestBuilder {
    count: Option<i64>,
    offset: Option<i64>,
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    before_create_time: Option<DateTime<FixedOffset>>,
    since_create_time: Option<DateTime<FixedOffset>>,
    before_start_time: Option<DateTime<FixedOffset>>,
    since_start_time: Option<DateTime<FixedOffset>>,
    status: Option<ListAutomationsRequestStatus>,
}

impl AutomationsListQueryRequestBuilder {
    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
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

    pub fn before_create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before_create_time = Some(value);
        self
    }

    pub fn since_create_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_create_time = Some(value);
        self
    }

    pub fn before_start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before_start_time = Some(value);
        self
    }

    pub fn since_start_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_start_time = Some(value);
        self
    }

    pub fn status(mut self, value: ListAutomationsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](AutomationsListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](AutomationsListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<AutomationsListQueryRequest, BuildError> {
        Ok(AutomationsListQueryRequest {
            count: self.count,
            offset: self.offset,
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            before_create_time: self.before_create_time,
            since_create_time: self.since_create_time,
            before_start_time: self.before_start_time,
            since_start_time: self.since_start_time,
            status: self.status,
        })
    }
}

