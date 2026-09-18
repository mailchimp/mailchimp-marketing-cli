pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// High level reporting stats for an outreach.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FacebookAdReportSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub conversion_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecommerce: Option<FacebookAdReportSummaryEcommerce>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagements: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub impressions: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub proxy_excluded_open_rate: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_excluded_opens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_excluded_unique_opens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reach: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_clicks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_sent: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_visits: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visits: Option<i64>,
}

impl FacebookAdReportSummary {
    pub fn builder() -> FacebookAdReportSummaryBuilder {
        <FacebookAdReportSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdReportSummaryBuilder {
    click_rate: Option<f64>,
    clicks: Option<i64>,
    conversion_rate: Option<f64>,
    ecommerce: Option<FacebookAdReportSummaryEcommerce>,
    engagements: Option<i64>,
    impressions: Option<f64>,
    open_rate: Option<f64>,
    opens: Option<i64>,
    proxy_excluded_open_rate: Option<f64>,
    proxy_excluded_opens: Option<i64>,
    proxy_excluded_unique_opens: Option<i64>,
    reach: Option<i64>,
    subscriber_clicks: Option<i64>,
    subscribes: Option<i64>,
    total_sent: Option<i64>,
    unique_opens: Option<i64>,
    unique_visits: Option<i64>,
    visits: Option<i64>,
}

impl FacebookAdReportSummaryBuilder {
    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
        self
    }

    pub fn conversion_rate(mut self, value: f64) -> Self {
        self.conversion_rate = Some(value);
        self
    }

    pub fn ecommerce(mut self, value: FacebookAdReportSummaryEcommerce) -> Self {
        self.ecommerce = Some(value);
        self
    }

    pub fn engagements(mut self, value: i64) -> Self {
        self.engagements = Some(value);
        self
    }

    pub fn impressions(mut self, value: f64) -> Self {
        self.impressions = Some(value);
        self
    }

    pub fn open_rate(mut self, value: f64) -> Self {
        self.open_rate = Some(value);
        self
    }

    pub fn opens(mut self, value: i64) -> Self {
        self.opens = Some(value);
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

    pub fn reach(mut self, value: i64) -> Self {
        self.reach = Some(value);
        self
    }

    pub fn subscriber_clicks(mut self, value: i64) -> Self {
        self.subscriber_clicks = Some(value);
        self
    }

    pub fn subscribes(mut self, value: i64) -> Self {
        self.subscribes = Some(value);
        self
    }

    pub fn total_sent(mut self, value: i64) -> Self {
        self.total_sent = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    pub fn unique_visits(mut self, value: i64) -> Self {
        self.unique_visits = Some(value);
        self
    }

    pub fn visits(mut self, value: i64) -> Self {
        self.visits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdReportSummary`].
    pub fn build(self) -> Result<FacebookAdReportSummary, BuildError> {
        Ok(FacebookAdReportSummary {
            click_rate: self.click_rate,
            clicks: self.clicks,
            conversion_rate: self.conversion_rate,
            ecommerce: self.ecommerce,
            engagements: self.engagements,
            impressions: self.impressions,
            open_rate: self.open_rate,
            opens: self.opens,
            proxy_excluded_open_rate: self.proxy_excluded_open_rate,
            proxy_excluded_opens: self.proxy_excluded_opens,
            proxy_excluded_unique_opens: self.proxy_excluded_unique_opens,
            reach: self.reach,
            subscriber_clicks: self.subscriber_clicks,
            subscribes: self.subscribes,
            total_sent: self.total_sent,
            unique_opens: self.unique_opens,
            unique_visits: self.unique_visits,
            visits: self.visits,
        })
    }
}
