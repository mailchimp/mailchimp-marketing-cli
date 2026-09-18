pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Updates on campaigns in the process of sending.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignReportDeliveryStatus {
    /// Whether a campaign send can be canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_cancel: Option<bool>,
    /// The total number of emails canceled for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_canceled: Option<i64>,
    /// The total number of emails confirmed sent for this campaign so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_sent: Option<i64>,
    /// Whether Campaign Delivery Status is enabled for this account and campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The current state of a campaign delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignReportDeliveryStatusStatus>,
}

impl CampaignReportDeliveryStatus {
    pub fn builder() -> CampaignReportDeliveryStatusBuilder {
        <CampaignReportDeliveryStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignReportDeliveryStatusBuilder {
    can_cancel: Option<bool>,
    emails_canceled: Option<i64>,
    emails_sent: Option<i64>,
    enabled: Option<bool>,
    status: Option<CampaignReportDeliveryStatusStatus>,
}

impl CampaignReportDeliveryStatusBuilder {
    pub fn can_cancel(mut self, value: bool) -> Self {
        self.can_cancel = Some(value);
        self
    }

    pub fn emails_canceled(mut self, value: i64) -> Self {
        self.emails_canceled = Some(value);
        self
    }

    pub fn emails_sent(mut self, value: i64) -> Self {
        self.emails_sent = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(mut self, value: CampaignReportDeliveryStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignReportDeliveryStatus`].
    pub fn build(self) -> Result<CampaignReportDeliveryStatus, BuildError> {
        Ok(CampaignReportDeliveryStatus {
            can_cancel: self.can_cancel,
            emails_canceled: self.emails_canceled,
            emails_sent: self.emails_sent,
            enabled: self.enabled,
            status: self.status,
        })
    }
}
