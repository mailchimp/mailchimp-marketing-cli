pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An object describing the open activity for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignReportOpens {
    /// The date and time of the last recorded open in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_open: Option<DateTime<FixedOffset>>,
    /// The number of unique opens for a campaign divided by the total number of successful deliveries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_rate: Option<f64>,
    /// The total number of opens for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens_total: Option<i64>,
    /// The average unique open rate for a campaign, excluding opens from email clients that use proxies.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub proxy_excluded_open_rate: Option<f64>,
    /// The total number of opens for a campaign, excluding opens from email clients that use proxies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_excluded_opens: Option<i64>,
    /// The total number of unique opens for a campaign, excluding opens from email clients that use proxies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_excluded_unique_opens: Option<i64>,
    /// The total number of unique opens for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
}

impl CampaignReportOpens {
    pub fn builder() -> CampaignReportOpensBuilder {
        <CampaignReportOpensBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportOpensBuilder {
    last_open: Option<DateTime<FixedOffset>>,
    open_rate: Option<f64>,
    opens_total: Option<i64>,
    proxy_excluded_open_rate: Option<f64>,
    proxy_excluded_opens: Option<i64>,
    proxy_excluded_unique_opens: Option<i64>,
    unique_opens: Option<i64>,
}

impl CampaignReportOpensBuilder {
    pub fn last_open(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_open = Some(value);
        self
    }

    pub fn open_rate(mut self, value: f64) -> Self {
        self.open_rate = Some(value);
        self
    }

    pub fn opens_total(mut self, value: i64) -> Self {
        self.opens_total = Some(value);
        self
    }

    pub fn proxy_excluded_open_rate(mut self, value: f64) -> Self {
        self.proxy_excluded_open_rate = Some(value);
        self
    }

    pub fn proxy_excluded_opens(mut self, value: i64) -> Self {
        self.proxy_excluded_opens = Some(value);
        self
    }

    pub fn proxy_excluded_unique_opens(mut self, value: i64) -> Self {
        self.proxy_excluded_unique_opens = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportOpens`].
    pub fn build(self) -> Result<CampaignReportOpens, BuildError> {
        Ok(CampaignReportOpens {
            last_open: self.last_open,
            open_rate: self.open_rate,
            opens_total: self.opens_total,
            proxy_excluded_open_rate: self.proxy_excluded_open_rate,
            proxy_excluded_opens: self.proxy_excluded_opens,
            proxy_excluded_unique_opens: self.proxy_excluded_unique_opens,
            unique_opens: self.unique_opens,
        })
    }
}
