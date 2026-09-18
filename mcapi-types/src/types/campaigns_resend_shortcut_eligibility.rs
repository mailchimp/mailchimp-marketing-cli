pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Determines if the campaign qualifies for the Campaign Resend Shortcuts. Only included when query parameter `include_resend_shortcuts` is `true`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsResendShortcutEligibility {
    /// Determines if the campaign qualifies to be resent to new subscribers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_new_subscribers: Option<CampaignsResendShortcutEligibilityToNewSubscribers>,
    /// Determines if the campaign qualifies to be resent to non-clickers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_non_clickers: Option<CampaignsResendShortcutEligibilityToNonClickers>,
    /// Determines if the campaign qualifies to be resent to non-openers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_non_openers: Option<CampaignsResendShortcutEligibilityToNonOpeners>,
    /// Determines if the campaign qualifies to be resent to non-purchasers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_non_purchasers: Option<CampaignsResendShortcutEligibilityToNonPurchasers>,
}

impl CampaignsResendShortcutEligibility {
    pub fn builder() -> CampaignsResendShortcutEligibilityBuilder {
        <CampaignsResendShortcutEligibilityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsResendShortcutEligibilityBuilder {
    to_new_subscribers: Option<CampaignsResendShortcutEligibilityToNewSubscribers>,
    to_non_clickers: Option<CampaignsResendShortcutEligibilityToNonClickers>,
    to_non_openers: Option<CampaignsResendShortcutEligibilityToNonOpeners>,
    to_non_purchasers: Option<CampaignsResendShortcutEligibilityToNonPurchasers>,
}

impl CampaignsResendShortcutEligibilityBuilder {
    pub fn to_new_subscribers(mut self, value: CampaignsResendShortcutEligibilityToNewSubscribers) -> Self {
        self.to_new_subscribers = Some(value);
        self
    }

    pub fn to_non_clickers(mut self, value: CampaignsResendShortcutEligibilityToNonClickers) -> Self {
        self.to_non_clickers = Some(value);
        self
    }

    pub fn to_non_openers(mut self, value: CampaignsResendShortcutEligibilityToNonOpeners) -> Self {
        self.to_non_openers = Some(value);
        self
    }

    pub fn to_non_purchasers(mut self, value: CampaignsResendShortcutEligibilityToNonPurchasers) -> Self {
        self.to_non_purchasers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsResendShortcutEligibility`].
    pub fn build(self) -> Result<CampaignsResendShortcutEligibility, BuildError> {
        Ok(CampaignsResendShortcutEligibility {
            to_new_subscribers: self.to_new_subscribers,
            to_non_clickers: self.to_non_clickers,
            to_non_openers: self.to_non_openers,
            to_non_purchasers: self.to_non_purchasers,
        })
    }
}
