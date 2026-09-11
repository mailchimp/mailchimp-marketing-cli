pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Determines if the campaign qualifies to be resent to non-clickers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsResendShortcutEligibilityToNonClickers {
    /// Determines if the campaign qualifies to be resent to this segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_eligible: Option<bool>,
    /// The reason the campaign is not eligible to be resent to this segment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CampaignsResendShortcutEligibilityToNonClickers {
    pub fn builder() -> CampaignsResendShortcutEligibilityToNonClickersBuilder {
        <CampaignsResendShortcutEligibilityToNonClickersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsResendShortcutEligibilityToNonClickersBuilder {
    is_eligible: Option<bool>,
    reason: Option<String>,
}

impl CampaignsResendShortcutEligibilityToNonClickersBuilder {
    pub fn is_eligible(mut self, value: bool) -> Self {
        self.is_eligible = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignsResendShortcutEligibilityToNonClickers`].
    pub fn build(self) -> Result<CampaignsResendShortcutEligibilityToNonClickers, BuildError> {
        Ok(CampaignsResendShortcutEligibilityToNonClickers {
            is_eligible: self.is_eligible,
            reason: self.reason,
        })
    }
}
