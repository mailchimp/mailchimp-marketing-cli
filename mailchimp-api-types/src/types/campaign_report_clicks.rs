pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An object describing the click activity for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignReportClicks {
    /// The number of unique clicks divided by the total number of successful deliveries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    /// The total number of clicks for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks_total: Option<i64>,
    /// The date and time of the last recorded click for the campaign in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_click: Option<DateTime<FixedOffset>>,
    /// The total number of unique clicks for links across a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_clicks: Option<i64>,
    /// The total number of subscribers who clicked on a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_subscriber_clicks: Option<i64>,
}

impl CampaignReportClicks {
    pub fn builder() -> CampaignReportClicksBuilder {
        <CampaignReportClicksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportClicksBuilder {
    click_rate: Option<f64>,
    clicks_total: Option<i64>,
    last_click: Option<DateTime<FixedOffset>>,
    unique_clicks: Option<i64>,
    unique_subscriber_clicks: Option<i64>,
}

impl CampaignReportClicksBuilder {
    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn clicks_total(mut self, value: i64) -> Self {
        self.clicks_total = Some(value);
        self
    }

    pub fn last_click(mut self, value: DateTime<FixedOffset>) -> Self {
        self.last_click = Some(value);
        self
    }

    pub fn unique_clicks(mut self, value: i64) -> Self {
        self.unique_clicks = Some(value);
        self
    }

    pub fn unique_subscriber_clicks(mut self, value: i64) -> Self {
        self.unique_subscriber_clicks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportClicks`].
    pub fn build(self) -> Result<CampaignReportClicks, BuildError> {
        Ok(CampaignReportClicks {
            click_rate: self.click_rate,
            clicks_total: self.clicks_total,
            last_click: self.last_click,
            unique_clicks: self.unique_clicks,
            unique_subscriber_clicks: self.unique_subscriber_clicks,
        })
    }
}
