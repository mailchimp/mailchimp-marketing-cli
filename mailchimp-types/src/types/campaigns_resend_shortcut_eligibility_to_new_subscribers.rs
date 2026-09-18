pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Determines if the campaign qualifies to be resent to new subscribers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsResendShortcutEligibilityToNewSubscribers {
    /// Determines if the campaign qualifies to be resent to this segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eligible: Option<bool>,
    /// The reason the campaign is not eligible to be resent to this segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CampaignsResendShortcutEligibilityToNewSubscribers {
    pub fn builder() -> CampaignsResendShortcutEligibilityToNewSubscribersBuilder {
        <CampaignsResendShortcutEligibilityToNewSubscribersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsResendShortcutEligibilityToNewSubscribersBuilder {
    is_eligible: Option<bool>,
    reason: Option<String>,
}

impl CampaignsResendShortcutEligibilityToNewSubscribersBuilder {
    pub fn is_eligible(mut self, value: bool) -> Self {
        self.is_eligible = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignsResendShortcutEligibilityToNewSubscribers`].
    pub fn build(self) -> Result<CampaignsResendShortcutEligibilityToNewSubscribers, BuildError> {
        Ok(CampaignsResendShortcutEligibilityToNewSubscribers {
            is_eligible: self.is_eligible,
            reason: self.reason,
        })
    }
}
