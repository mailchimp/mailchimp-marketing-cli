pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportingFacebookAdReportSummaryExtendedAt {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datetime: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl ReportingFacebookAdReportSummaryExtendedAt {
    pub fn builder() -> ReportingFacebookAdReportSummaryExtendedAtBuilder {
        <ReportingFacebookAdReportSummaryExtendedAtBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdReportSummaryExtendedAtBuilder {
    datetime: Option<String>,
    timezone: Option<String>,
}

impl ReportingFacebookAdReportSummaryExtendedAtBuilder {
    pub fn datetime(mut self, value: impl Into<String>) -> Self {
        self.datetime = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdReportSummaryExtendedAt`].
    pub fn build(self) -> Result<ReportingFacebookAdReportSummaryExtendedAt, BuildError> {
        Ok(ReportingFacebookAdReportSummaryExtendedAt {
            datetime: self.datetime,
            timezone: self.timezone,
        })
    }
}
