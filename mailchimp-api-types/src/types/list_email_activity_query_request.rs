pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-email-activity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEmailActivityQueryRequest {
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
    /// Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_bots: Option<bool>,
}

impl ListEmailActivityQueryRequest {
    pub fn builder() -> ListEmailActivityQueryRequestBuilder {
        <ListEmailActivityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEmailActivityQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    since: Option<String>,
    filter_bots: Option<bool>,
}

impl ListEmailActivityQueryRequestBuilder {
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

    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    pub fn filter_bots(mut self, value: bool) -> Self {
        self.filter_bots = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEmailActivityQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListEmailActivityQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListEmailActivityQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListEmailActivityQueryRequest, BuildError> {
        Ok(ListEmailActivityQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            since: self.since,
            filter_bots: self.filter_bots,
        })
    }
}

