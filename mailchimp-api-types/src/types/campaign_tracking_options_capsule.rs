pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deprecated
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignTrackingOptionsCapsule {
    /// Update contact notes for a campaign based on subscriber email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<bool>,
}

impl CampaignTrackingOptionsCapsule {
    pub fn builder() -> CampaignTrackingOptionsCapsuleBuilder {
        <CampaignTrackingOptionsCapsuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignTrackingOptionsCapsuleBuilder {
    notes: Option<bool>,
}

impl CampaignTrackingOptionsCapsuleBuilder {
    pub fn notes(mut self, value: bool) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignTrackingOptionsCapsule`].
    pub fn build(self) -> Result<CampaignTrackingOptionsCapsule, BuildError> {
        Ok(CampaignTrackingOptionsCapsule {
            notes: self.notes,
        })
    }
}
