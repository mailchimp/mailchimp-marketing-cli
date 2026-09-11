pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportingFacebookAdReportSummaryCostPerClick {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl ReportingFacebookAdReportSummaryCostPerClick {
    pub fn builder() -> ReportingFacebookAdReportSummaryCostPerClickBuilder {
        <ReportingFacebookAdReportSummaryCostPerClickBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdReportSummaryCostPerClickBuilder {
    amount: Option<f64>,
    currency_code: Option<String>,
}

impl ReportingFacebookAdReportSummaryCostPerClickBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdReportSummaryCostPerClick`].
    pub fn build(self) -> Result<ReportingFacebookAdReportSummaryCostPerClick, BuildError> {
        Ok(ReportingFacebookAdReportSummaryCostPerClick {
            amount: self.amount,
            currency_code: self.currency_code,
        })
    }
}
