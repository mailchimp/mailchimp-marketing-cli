pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// E-Commerce stats for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignReportEcommerce {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The total orders for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_orders: Option<i64>,
    /// The total revenue for a campaign. Calculated as the sum of all order totals minus shipping and tax totals.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_revenue: Option<f64>,
    /// The total spent for a campaign. Calculated as the sum of all order totals with no deductions.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_spent: Option<f64>,
}

impl CampaignReportEcommerce {
    pub fn builder() -> CampaignReportEcommerceBuilder {
        <CampaignReportEcommerceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportEcommerceBuilder {
    currency_code: Option<String>,
    total_orders: Option<i64>,
    total_revenue: Option<f64>,
    total_spent: Option<f64>,
}

impl CampaignReportEcommerceBuilder {
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

    pub fn total_spent(mut self, value: f64) -> Self {
        self.total_spent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportEcommerce`].
    pub fn build(self) -> Result<CampaignReportEcommerce, BuildError> {
        Ok(CampaignReportEcommerce {
            currency_code: self.currency_code,
            total_orders: self.total_orders,
            total_revenue: self.total_revenue,
            total_spent: self.total_spent,
        })
    }
}
