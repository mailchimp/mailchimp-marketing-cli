pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// E-Commerce stats for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignsReportSummaryEcommerce {
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

impl CampaignsReportSummaryEcommerce {
    pub fn builder() -> CampaignsReportSummaryEcommerceBuilder {
        <CampaignsReportSummaryEcommerceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsReportSummaryEcommerceBuilder {
    total_orders: Option<i64>,
    total_revenue: Option<f64>,
    total_spent: Option<f64>,
}

impl CampaignsReportSummaryEcommerceBuilder {
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

    /// Consumes the builder and constructs a [`CampaignsReportSummaryEcommerce`].
    pub fn build(self) -> Result<CampaignsReportSummaryEcommerce, BuildError> {
        Ok(CampaignsReportSummaryEcommerce {
            total_orders: self.total_orders,
            total_revenue: self.total_revenue,
            total_spent: self.total_spent,
        })
    }
}
