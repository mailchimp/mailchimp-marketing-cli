pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-abuse-report
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportsGetAbuseReportQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
}

impl ReportsGetAbuseReportQueryRequest {
    pub fn builder() -> ReportsGetAbuseReportQueryRequestBuilder {
        <ReportsGetAbuseReportQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportsGetAbuseReportQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
}

impl ReportsGetAbuseReportQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportsGetAbuseReportQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ReportsGetAbuseReportQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ReportsGetAbuseReportQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ReportsGetAbuseReportQueryRequest, BuildError> {
        Ok(ReportsGetAbuseReportQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
        })
    }
}

