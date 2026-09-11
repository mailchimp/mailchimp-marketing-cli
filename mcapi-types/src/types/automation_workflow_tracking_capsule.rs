pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deprecated
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowTrackingCapsule {
    /// Update contact notes for a campaign based on a subscriber's email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<bool>,
}

impl AutomationWorkflowTrackingCapsule {
    pub fn builder() -> AutomationWorkflowTrackingCapsuleBuilder {
        <AutomationWorkflowTrackingCapsuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowTrackingCapsuleBuilder {
    notes: Option<bool>,
}

impl AutomationWorkflowTrackingCapsuleBuilder {
    pub fn notes(mut self, value: bool) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowTrackingCapsule`].
    pub fn build(self) -> Result<AutomationWorkflowTrackingCapsule, BuildError> {
        Ok(AutomationWorkflowTrackingCapsule {
            notes: self.notes,
        })
    }
}
