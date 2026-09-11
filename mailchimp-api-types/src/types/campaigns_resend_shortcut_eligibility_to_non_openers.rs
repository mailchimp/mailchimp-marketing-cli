pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Determines if the campaign qualifies to be resent to non-openers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsResendShortcutEligibilityToNonOpeners {
    /// Determines if the campaign qualifies to be resent to this segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eligible: Option<bool>,
    /// The reason the campaign is not eligible to be resent to this segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CampaignsResendShortcutEligibilityToNonOpeners {
    pub fn builder() -> CampaignsResendShortcutEligibilityToNonOpenersBuilder {
        <CampaignsResendShortcutEligibilityToNonOpenersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsResendShortcutEligibilityToNonOpenersBuilder {
    is_eligible: Option<bool>,
    reason: Option<String>,
}

impl CampaignsResendShortcutEligibilityToNonOpenersBuilder {
    pub fn is_eligible(mut self, value: bool) -> Self {
        self.is_eligible = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignsResendShortcutEligibilityToNonOpeners`].
    pub fn build(self) -> Result<CampaignsResendShortcutEligibilityToNonOpeners, BuildError> {
        Ok(CampaignsResendShortcutEligibilityToNonOpeners {
            is_eligible: self.is_eligible,
            reason: self.reason,
        })
    }
}
