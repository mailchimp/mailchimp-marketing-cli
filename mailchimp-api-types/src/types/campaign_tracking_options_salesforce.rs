pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deprecated
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignTrackingOptionsSalesforce {
    /// Create a campaign in a connected Salesforce account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<bool>,
    /// Update contact notes for a campaign based on subscriber email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<bool>,
}

impl CampaignTrackingOptionsSalesforce {
    pub fn builder() -> CampaignTrackingOptionsSalesforceBuilder {
        <CampaignTrackingOptionsSalesforceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignTrackingOptionsSalesforceBuilder {
    campaign: Option<bool>,
    notes: Option<bool>,
}

impl CampaignTrackingOptionsSalesforceBuilder {
    pub fn campaign(mut self, value: bool) -> Self {
        self.campaign = Some(value);
        self
    }

    pub fn notes(mut self, value: bool) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignTrackingOptionsSalesforce`].
    pub fn build(self) -> Result<CampaignTrackingOptionsSalesforce, BuildError> {
        Ok(CampaignTrackingOptionsSalesforce {
            campaign: self.campaign,
            notes: self.notes,
        })
    }
}
