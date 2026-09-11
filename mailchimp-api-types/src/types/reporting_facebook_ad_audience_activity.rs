pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReportingFacebookAdAudienceActivity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<Vec<ReportingFacebookAdAudienceActivityClicksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impressions: Option<Vec<ReportingFacebookAdAudienceActivityImpressionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revenue: Option<Vec<ReportingFacebookAdAudienceActivityRevenueItem>>,
}

impl ReportingFacebookAdAudienceActivity {
    pub fn builder() -> ReportingFacebookAdAudienceActivityBuilder {
        <ReportingFacebookAdAudienceActivityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdAudienceActivityBuilder {
    clicks: Option<Vec<ReportingFacebookAdAudienceActivityClicksItem>>,
    impressions: Option<Vec<ReportingFacebookAdAudienceActivityImpressionsItem>>,
    revenue: Option<Vec<ReportingFacebookAdAudienceActivityRevenueItem>>,
}

impl ReportingFacebookAdAudienceActivityBuilder {
    pub fn clicks(mut self, value: Vec<ReportingFacebookAdAudienceActivityClicksItem>) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn impressions(mut self, value: Vec<ReportingFacebookAdAudienceActivityImpressionsItem>) -> Self {
        self.impressions = Some(value);
        self
    }

    pub fn revenue(mut self, value: Vec<ReportingFacebookAdAudienceActivityRevenueItem>) -> Self {
        self.revenue = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdAudienceActivity`].
    pub fn build(self) -> Result<ReportingFacebookAdAudienceActivity, BuildError> {
        Ok(ReportingFacebookAdAudienceActivity {
            clicks: self.clicks,
            impressions: self.impressions,
            revenue: self.revenue,
        })
    }
}
