pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-email-activity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetEmailActivityQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// When true, exclude automated bot and Apple Mail Privacy Protection (MPP) proxy activity so the returned activity reflects human-only opens and clicks, matching the in-app Recipient Activity view. Filtering removes events from a member's activity, but never removes the member from the response. Defaults to false (all activity).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_bots: Option<bool>,
}

impl GetEmailActivityQueryRequest {
    pub fn builder() -> GetEmailActivityQueryRequestBuilder {
        <GetEmailActivityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEmailActivityQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    since: Option<String>,
    filter_bots: Option<bool>,
}

impl GetEmailActivityQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
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

    /// Consumes the builder and constructs a [`GetEmailActivityQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](GetEmailActivityQueryRequestBuilder::fields)
    /// - [`exclude_fields`](GetEmailActivityQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<GetEmailActivityQueryRequest, BuildError> {
        Ok(GetEmailActivityQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            since: self.since,
            filter_bots: self.filter_bots,
        })
    }
}

