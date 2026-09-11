pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsResendShortcutUsageShortcutCampaignsItem {
    /// Unique ID for the resent campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date and time a resent campaign was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub send_time: Option<DateTime<FixedOffset>>,
    /// Which campaign resend shortcut was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_type: Option<CampaignsResendShortcutUsageShortcutCampaignsItemShortcutType>,
    /// The current status of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignsResendShortcutUsageShortcutCampaignsItemStatus>,
    /// The ID for the resent campaign used in the Mailchimp web application. View this campaign in your Mailchimp account at `https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl CampaignsResendShortcutUsageShortcutCampaignsItem {
    pub fn builder() -> CampaignsResendShortcutUsageShortcutCampaignsItemBuilder {
        <CampaignsResendShortcutUsageShortcutCampaignsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsResendShortcutUsageShortcutCampaignsItemBuilder {
    id: Option<String>,
    send_time: Option<DateTime<FixedOffset>>,
    shortcut_type: Option<CampaignsResendShortcutUsageShortcutCampaignsItemShortcutType>,
    status: Option<CampaignsResendShortcutUsageShortcutCampaignsItemStatus>,
    web_id: Option<i64>,
}

impl CampaignsResendShortcutUsageShortcutCampaignsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn send_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.send_time = Some(value);
        self
    }

    pub fn shortcut_type(mut self, value: CampaignsResendShortcutUsageShortcutCampaignsItemShortcutType) -> Self {
        self.shortcut_type = Some(value);
        self
    }

    pub fn status(mut self, value: CampaignsResendShortcutUsageShortcutCampaignsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsResendShortcutUsageShortcutCampaignsItem`].
    pub fn build(self) -> Result<CampaignsResendShortcutUsageShortcutCampaignsItem, BuildError> {
        Ok(CampaignsResendShortcutUsageShortcutCampaignsItem {
            id: self.id,
            send_time: self.send_time,
            shortcut_type: self.shortcut_type,
            status: self.status,
            web_id: self.web_id,
        })
    }
}
