pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// For sent campaigns, a summary of opens and clicks.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AutomationWorkflowEmailReportSummary {
    /// The number of unique clicks divided by the total number of successful deliveries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub click_rate: Option<f64>,
    /// The total number of clicks for an campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    /// The number of unique opens divided by the total number of successful deliveries.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub open_rate: Option<f64>,
    /// The total number of opens for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    /// The number of unique clicks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriber_clicks: Option<i64>,
    /// The number of unique opens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_opens: Option<i64>,
}

impl AutomationWorkflowEmailReportSummary {
    pub fn builder() -> AutomationWorkflowEmailReportSummaryBuilder {
        <AutomationWorkflowEmailReportSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailReportSummaryBuilder {
    click_rate: Option<f64>,
    clicks: Option<i64>,
    open_rate: Option<f64>,
    opens: Option<i64>,
    subscriber_clicks: Option<i64>,
    unique_opens: Option<i64>,
}

impl AutomationWorkflowEmailReportSummaryBuilder {
    pub fn click_rate(mut self, value: f64) -> Self {
        self.click_rate = Some(value);
        self
    }

    pub fn clicks(mut self, value: i64) -> Self {
        self.clicks = Some(value);
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

    pub fn subscriber_clicks(mut self, value: i64) -> Self {
        self.subscriber_clicks = Some(value);
        self
    }

    pub fn unique_opens(mut self, value: i64) -> Self {
        self.unique_opens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmailReportSummary`].
    pub fn build(self) -> Result<AutomationWorkflowEmailReportSummary, BuildError> {
        Ok(AutomationWorkflowEmailReportSummary {
            click_rate: self.click_rate,
            clicks: self.clicks,
            open_rate: self.open_rate,
            opens: self.opens,
            subscriber_clicks: self.subscriber_clicks,
            unique_opens: self.unique_opens,
        })
    }
}
