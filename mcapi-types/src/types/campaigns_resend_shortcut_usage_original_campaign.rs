pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The original campaign that was resent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsResendShortcutUsageOriginalCampaign {
    /// ID for the resent campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Which campaign resend shortcut was used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_type: Option<CampaignsResendShortcutUsageOriginalCampaignShortcutType>,
    /// The title of the original campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The ID for the resent campaign used in the Mailchimp web application. View this campaign in your Mailchimp account at `https://{dc}.admin.mailchimp.com/campaigns/show/?id={web_id}`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_id: Option<i64>,
}

impl CampaignsResendShortcutUsageOriginalCampaign {
    pub fn builder() -> CampaignsResendShortcutUsageOriginalCampaignBuilder {
        <CampaignsResendShortcutUsageOriginalCampaignBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsResendShortcutUsageOriginalCampaignBuilder {
    id: Option<String>,
    shortcut_type: Option<CampaignsResendShortcutUsageOriginalCampaignShortcutType>,
    title: Option<String>,
    web_id: Option<i64>,
}

impl CampaignsResendShortcutUsageOriginalCampaignBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn shortcut_type(mut self, value: CampaignsResendShortcutUsageOriginalCampaignShortcutType) -> Self {
        self.shortcut_type = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn web_id(mut self, value: i64) -> Self {
        self.web_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsResendShortcutUsageOriginalCampaign`].
    pub fn build(self) -> Result<CampaignsResendShortcutUsageOriginalCampaign, BuildError> {
        Ok(CampaignsResendShortcutUsageOriginalCampaign {
            id: self.id,
            shortcut_type: self.shortcut_type,
            title: self.title,
            web_id: self.web_id,
        })
    }
}
