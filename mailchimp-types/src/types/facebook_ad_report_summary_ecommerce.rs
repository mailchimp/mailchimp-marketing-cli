pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FacebookAdReportSummaryEcommerce {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_order_revenue: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_revenue: Option<f64>,
}

impl FacebookAdReportSummaryEcommerce {
    pub fn builder() -> FacebookAdReportSummaryEcommerceBuilder {
        <FacebookAdReportSummaryEcommerceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdReportSummaryEcommerceBuilder {
    average_order_revenue: Option<f64>,
    currency_code: Option<String>,
    total_revenue: Option<f64>,
}

impl FacebookAdReportSummaryEcommerceBuilder {
    pub fn average_order_revenue(mut self, value: f64) -> Self {
        self.average_order_revenue = Some(value);
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn total_revenue(mut self, value: f64) -> Self {
        self.total_revenue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdReportSummaryEcommerce`].
    pub fn build(self) -> Result<FacebookAdReportSummaryEcommerce, BuildError> {
        Ok(FacebookAdReportSummaryEcommerce {
            average_order_revenue: self.average_order_revenue,
            currency_code: self.currency_code,
            total_revenue: self.total_revenue,
        })
    }
}
