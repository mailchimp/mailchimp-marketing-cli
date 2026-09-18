pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deprecated
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowTrackingSalesforce {
    /// Create a campaign in a connected Salesforce account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<bool>,
    /// Update contact notes for a campaign based on a subscriber's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<bool>,
}

impl AutomationWorkflowTrackingSalesforce {
    pub fn builder() -> AutomationWorkflowTrackingSalesforceBuilder {
        <AutomationWorkflowTrackingSalesforceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowTrackingSalesforceBuilder {
    campaign: Option<bool>,
    notes: Option<bool>,
}

impl AutomationWorkflowTrackingSalesforceBuilder {
    pub fn campaign(mut self, value: bool) -> Self {
        self.campaign = Some(value);
        self
    }

    pub fn notes(mut self, value: bool) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowTrackingSalesforce`].
    pub fn build(self) -> Result<AutomationWorkflowTrackingSalesforce, BuildError> {
        Ok(AutomationWorkflowTrackingSalesforce {
            campaign: self.campaign,
            notes: self.notes,
        })
    }
}
