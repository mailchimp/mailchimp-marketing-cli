pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportTimeseriesItem {
    /// The number of emails sent in the timeseries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// The number of unique opens in the timeseries, excluding opens from email clients that use proxies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_excluded_unique_opens: Option<i64>,
    /// The number of clicks in the timeseries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipients_clicks: Option<i64>,
    /// The date and time for the series in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
    /// The number of unique opens in the timeseries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
}

impl CampaignReportTimeseriesItem {
    pub fn builder() -> CampaignReportTimeseriesItemBuilder {
        <CampaignReportTimeseriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportTimeseriesItemBuilder {
    emails_sent: Option<i64>,
    proxy_excluded_unique_opens: Option<i64>,
    recipients_clicks: Option<i64>,
    timestamp: Option<DateTime<FixedOffset>>,
    unique_opens: Option<i64>,
}

impl CampaignReportTimeseriesItemBuilder {
    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn proxy_excluded_unique_opens(mut self, value: i64) -> Self {
        self.proxy_excluded_unique_opens = Some(value);
        self
    }

    pub fn recipients_clicks(mut self, value: i64) -> Self {
        self.recipients_clicks = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportTimeseriesItem`].
    pub fn build(self) -> Result<CampaignReportTimeseriesItem, BuildError> {
        Ok(CampaignReportTimeseriesItem {
            emails_sent: self.emails_sent,
            proxy_excluded_unique_opens: self.proxy_excluded_unique_opens,
            recipients_clicks: self.recipients_clicks,
            timestamp: self.timestamp,
            unique_opens: self.unique_opens,
        })
    }
}
