pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LandingPageReportEcommerce {
    /// The average order revenue of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_order_revenue: Option<f64>,
    /// The user's currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The total number of orders associated with this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_orders: Option<i64>,
    /// The total revenue of this landing page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_revenue: Option<f64>,
}

impl LandingPageReportEcommerce {
    pub fn builder() -> LandingPageReportEcommerceBuilder {
        <LandingPageReportEcommerceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LandingPageReportEcommerceBuilder {
    average_order_revenue: Option<f64>,
    currency_code: Option<String>,
    total_orders: Option<i64>,
    total_revenue: Option<f64>,
}

impl LandingPageReportEcommerceBuilder {
    pub fn average_order_revenue(mut self, value: f64) -> Self {
        self.average_order_revenue = Some(value);
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn total_orders(mut self, value: i64) -> Self {
        self.total_orders = Some(value);
        self
    }

    pub fn total_revenue(mut self, value: f64) -> Self {
        self.total_revenue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LandingPageReportEcommerce`].
    pub fn build(self) -> Result<LandingPageReportEcommerce, BuildError> {
        Ok(LandingPageReportEcommerce {
            average_order_revenue: self.average_order_revenue,
            currency_code: self.currency_code,
            total_orders: self.total_orders,
            total_revenue: self.total_revenue,
        })
    }
}
