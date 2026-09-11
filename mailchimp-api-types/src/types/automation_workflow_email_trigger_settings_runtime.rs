pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A workflow's runtime settings for an Automation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowEmailTriggerSettingsRuntime {
    /// The days an Automation workflow can send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<Vec<AutomationWorkflowEmailTriggerSettingsRuntimeDaysItem>>,
    /// The hours an Automation workflow can send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<AutomationWorkflowEmailTriggerSettingsRuntimeHours>,
}

impl AutomationWorkflowEmailTriggerSettingsRuntime {
    pub fn builder() -> AutomationWorkflowEmailTriggerSettingsRuntimeBuilder {
        <AutomationWorkflowEmailTriggerSettingsRuntimeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailTriggerSettingsRuntimeBuilder {
    days: Option<Vec<AutomationWorkflowEmailTriggerSettingsRuntimeDaysItem>>,
    hours: Option<AutomationWorkflowEmailTriggerSettingsRuntimeHours>,
}

impl AutomationWorkflowEmailTriggerSettingsRuntimeBuilder {
    pub fn days(mut self, value: Vec<AutomationWorkflowEmailTriggerSettingsRuntimeDaysItem>) -> Self {
        self.days = Some(value);
        self
    }

    pub fn hours(mut self, value: AutomationWorkflowEmailTriggerSettingsRuntimeHours) -> Self {
        self.hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmailTriggerSettingsRuntime`].
    pub fn build(self) -> Result<AutomationWorkflowEmailTriggerSettingsRuntime, BuildError> {
        Ok(AutomationWorkflowEmailTriggerSettingsRuntime {
            days: self.days,
            hours: self.hours,
        })
    }
}
