pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about campaigns related through shortcuts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsResendShortcutUsage {
    /// The original campaign that was resent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_campaign: Option<CampaignsResendShortcutUsageOriginalCampaign>,
    /// Campaigns that were created from Campaign Resend Shortcuts for this campaign
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_campaigns: Option<Vec<CampaignsResendShortcutUsageShortcutCampaignsItem>>,
}

impl CampaignsResendShortcutUsage {
    pub fn builder() -> CampaignsResendShortcutUsageBuilder {
        <CampaignsResendShortcutUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsResendShortcutUsageBuilder {
    original_campaign: Option<CampaignsResendShortcutUsageOriginalCampaign>,
    shortcut_campaigns: Option<Vec<CampaignsResendShortcutUsageShortcutCampaignsItem>>,
}

impl CampaignsResendShortcutUsageBuilder {
    pub fn original_campaign(mut self, value: CampaignsResendShortcutUsageOriginalCampaign) -> Self {
        self.original_campaign = Some(value);
        self
    }

    pub fn shortcut_campaigns(mut self, value: Vec<CampaignsResendShortcutUsageShortcutCampaignsItem>) -> Self {
        self.shortcut_campaigns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsResendShortcutUsage`].
    pub fn build(self) -> Result<CampaignsResendShortcutUsage, BuildError> {
        Ok(CampaignsResendShortcutUsage {
            original_campaign: self.original_campaign,
            shortcut_campaigns: self.shortcut_campaigns,
        })
    }
}
