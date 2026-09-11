pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Deprecated
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowEmailTrackingCapsule {
    /// Update contact notes for a campaign based on a subscriber's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<bool>,
}

impl AutomationWorkflowEmailTrackingCapsule {
    pub fn builder() -> AutomationWorkflowEmailTrackingCapsuleBuilder {
        <AutomationWorkflowEmailTrackingCapsuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailTrackingCapsuleBuilder {
    notes: Option<bool>,
}

impl AutomationWorkflowEmailTrackingCapsuleBuilder {
    pub fn notes(mut self, value: bool) -> Self {
        self.notes = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmailTrackingCapsule`].
    pub fn build(self) -> Result<AutomationWorkflowEmailTrackingCapsule, BuildError> {
        Ok(AutomationWorkflowEmailTrackingCapsule {
            notes: self.notes,
        })
    }
}
