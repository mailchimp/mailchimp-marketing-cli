pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportingFacebookAdReportSummaryEcommerce {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_revenue: Option<f64>,
}

impl ReportingFacebookAdReportSummaryEcommerce {
    pub fn builder() -> ReportingFacebookAdReportSummaryEcommerceBuilder {
        <ReportingFacebookAdReportSummaryEcommerceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdReportSummaryEcommerceBuilder {
    currency_code: Option<String>,
    total_revenue: Option<f64>,
}

impl ReportingFacebookAdReportSummaryEcommerceBuilder {
    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn total_revenue(mut self, value: f64) -> Self {
        self.total_revenue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdReportSummaryEcommerce`].
    pub fn build(self) -> Result<ReportingFacebookAdReportSummaryEcommerce, BuildError> {
        Ok(ReportingFacebookAdReportSummaryEcommerce {
            currency_code: self.currency_code,
            total_revenue: self.total_revenue,
        })
    }
}
