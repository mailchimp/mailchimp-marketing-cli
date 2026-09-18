pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The average campaign statistics for your industry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignReportIndustryStats {
    /// The industry abuse rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub abuse_rate: Option<f64>,
    /// The industry bounce rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub bounce_rate: Option<f64>,
    /// The industry click rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    /// The industry open rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_rate: Option<f64>,
    /// The type of business industry associated with your account. For example: retail, education, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The industry unopened rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unopen_rate: Option<f64>,
    /// The industry unsubscribe rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unsub_rate: Option<f64>,
}

impl CampaignReportIndustryStats {
    pub fn builder() -> CampaignReportIndustryStatsBuilder {
        <CampaignReportIndustryStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportIndustryStatsBuilder {
    abuse_rate: Option<f64>,
    bounce_rate: Option<f64>,
    click_rate: Option<f64>,
    open_rate: Option<f64>,
    r#type: Option<String>,
    unopen_rate: Option<f64>,
    unsub_rate: Option<f64>,
}

impl CampaignReportIndustryStatsBuilder {
    pub fn abuse_rate(mut self, value: f64) -> Self {
        self.abuse_rate = Some(value);
        self
    }

    pub fn bounce_rate(mut self, value: f64) -> Self {
        self.bounce_rate = Some(value);
        self
    }

    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn open_rate(mut self, value: f64) -> Self {
        self.open_rate = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn unopen_rate(mut self, value: f64) -> Self {
        self.unopen_rate = Some(value);
        self
    }

    pub fn unsub_rate(mut self, value: f64) -> Self {
        self.unsub_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportIndustryStats`].
    pub fn build(self) -> Result<CampaignReportIndustryStats, BuildError> {
        Ok(CampaignReportIndustryStats {
            abuse_rate: self.abuse_rate,
            bounce_rate: self.bounce_rate,
            click_rate: self.click_rate,
            open_rate: self.open_rate,
            r#type: self.r#type,
            unopen_rate: self.unopen_rate,
            unsub_rate: self.unsub_rate,
        })
    }
}
