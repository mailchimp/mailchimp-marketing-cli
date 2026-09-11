pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The average campaign statistics for your list. This won't be present if we haven't calculated it yet for this list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignReportListStats {
    /// The average click rate (a percentage represented as a number between 0 and 100) per campaign for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    /// The average unique open rate (a percentage represented as a number between 0 and 100) per campaign for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_rate: Option<f64>,
    /// The average unique open rate (a percentage represented as a number between 0 and 100) per campaign for the list, excluding opens from email clients that use proxies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub proxy_excluded_open_rate: Option<f64>,
    /// The average number of subscriptions per month for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub sub_rate: Option<f64>,
    /// The average number of unsubscriptions per month for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub unsub_rate: Option<f64>,
}

impl CampaignReportListStats {
    pub fn builder() -> CampaignReportListStatsBuilder {
        <CampaignReportListStatsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportListStatsBuilder {
    click_rate: Option<f64>,
    open_rate: Option<f64>,
    proxy_excluded_open_rate: Option<f64>,
    sub_rate: Option<f64>,
    unsub_rate: Option<f64>,
}

impl CampaignReportListStatsBuilder {
    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn open_rate(mut self, value: f64) -> Self {
        self.open_rate = Some(value);
        self
    }

    pub fn proxy_excluded_open_rate(mut self, value: f64) -> Self {
        self.proxy_excluded_open_rate = Some(value);
        self
    }

    pub fn sub_rate(mut self, value: f64) -> Self {
        self.sub_rate = Some(value);
        self
    }

    pub fn unsub_rate(mut self, value: f64) -> Self {
        self.unsub_rate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportListStats`].
    pub fn build(self) -> Result<CampaignReportListStats, BuildError> {
        Ok(CampaignReportListStats {
            click_rate: self.click_rate,
            open_rate: self.open_rate,
            proxy_excluded_open_rate: self.proxy_excluded_open_rate,
            sub_rate: self.sub_rate,
            unsub_rate: self.unsub_rate,
        })
    }
}
