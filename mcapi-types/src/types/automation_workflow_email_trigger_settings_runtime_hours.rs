pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The hours an Automation workflow can send.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowEmailTriggerSettingsRuntimeHours {
    /// When to send the Automation email.
    pub r#type: AutomationWorkflowEmailTriggerSettingsRuntimeHoursType,
}

impl AutomationWorkflowEmailTriggerSettingsRuntimeHours {
    pub fn builder() -> AutomationWorkflowEmailTriggerSettingsRuntimeHoursBuilder {
        <AutomationWorkflowEmailTriggerSettingsRuntimeHoursBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailTriggerSettingsRuntimeHoursBuilder {
    r#type: Option<AutomationWorkflowEmailTriggerSettingsRuntimeHoursType>,
}

impl AutomationWorkflowEmailTriggerSettingsRuntimeHoursBuilder {
    pub fn r#type(mut self, value: AutomationWorkflowEmailTriggerSettingsRuntimeHoursType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmailTriggerSettingsRuntimeHours`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AutomationWorkflowEmailTriggerSettingsRuntimeHoursBuilder::r#type)
    pub fn build(self) -> Result<AutomationWorkflowEmailTriggerSettingsRuntimeHours, BuildError> {
        Ok(AutomationWorkflowEmailTriggerSettingsRuntimeHours {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
