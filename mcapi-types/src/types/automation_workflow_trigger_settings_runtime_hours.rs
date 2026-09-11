pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The hours an Automation workflow can send.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowTriggerSettingsRuntimeHours {
    /// When to send the Automation email.
    pub r#type: AutomationWorkflowTriggerSettingsRuntimeHoursType,
}

impl AutomationWorkflowTriggerSettingsRuntimeHours {
    pub fn builder() -> AutomationWorkflowTriggerSettingsRuntimeHoursBuilder {
        <AutomationWorkflowTriggerSettingsRuntimeHoursBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowTriggerSettingsRuntimeHoursBuilder {
    r#type: Option<AutomationWorkflowTriggerSettingsRuntimeHoursType>,
}

impl AutomationWorkflowTriggerSettingsRuntimeHoursBuilder {
    pub fn r#type(mut self, value: AutomationWorkflowTriggerSettingsRuntimeHoursType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowTriggerSettingsRuntimeHours`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AutomationWorkflowTriggerSettingsRuntimeHoursBuilder::r#type)
    pub fn build(self) -> Result<AutomationWorkflowTriggerSettingsRuntimeHours, BuildError> {
        Ok(AutomationWorkflowTriggerSettingsRuntimeHours {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
