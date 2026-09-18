pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A workflow's runtime settings for an Automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowTriggerSettingsRuntime {
    /// The days an Automation workflow can send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<AutomationWorkflowTriggerSettingsRuntimeDaysItem>>,
    /// The hours an Automation workflow can send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<AutomationWorkflowTriggerSettingsRuntimeHours>,
}

impl AutomationWorkflowTriggerSettingsRuntime {
    pub fn builder() -> AutomationWorkflowTriggerSettingsRuntimeBuilder {
        <AutomationWorkflowTriggerSettingsRuntimeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowTriggerSettingsRuntimeBuilder {
    days: Option<Vec<AutomationWorkflowTriggerSettingsRuntimeDaysItem>>,
    hours: Option<AutomationWorkflowTriggerSettingsRuntimeHours>,
}

impl AutomationWorkflowTriggerSettingsRuntimeBuilder {
    pub fn days(mut self, value: Vec<AutomationWorkflowTriggerSettingsRuntimeDaysItem>) -> Self {
        self.days = Some(value);
        self
    }

    pub fn hours(mut self, value: AutomationWorkflowTriggerSettingsRuntimeHours) -> Self {
        self.hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowTriggerSettingsRuntime`].
    pub fn build(self) -> Result<AutomationWorkflowTriggerSettingsRuntime, BuildError> {
        Ok(AutomationWorkflowTriggerSettingsRuntime {
            days: self.days,
            hours: self.hours,
        })
    }
}
