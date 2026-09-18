pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportingFacebookAdAudienceActivityRevenueItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub revenue: Option<f64>,
}

impl ReportingFacebookAdAudienceActivityRevenueItem {
    pub fn builder() -> ReportingFacebookAdAudienceActivityRevenueItemBuilder {
        <ReportingFacebookAdAudienceActivityRevenueItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdAudienceActivityRevenueItemBuilder {
    date: Option<String>,
    revenue: Option<f64>,
}

impl ReportingFacebookAdAudienceActivityRevenueItemBuilder {
    pub fn date(mut self, value: impl Into<String>) -> Self {
        self.date = Some(value.into());
        self
    }

    pub fn revenue(mut self, value: f64) -> Self {
        self.revenue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdAudienceActivityRevenueItem`].
    pub fn build(self) -> Result<ReportingFacebookAdAudienceActivityRevenueItem, BuildError> {
        Ok(ReportingFacebookAdAudienceActivityRevenueItem {
            date: self.date,
            revenue: self.revenue,
        })
    }
}
