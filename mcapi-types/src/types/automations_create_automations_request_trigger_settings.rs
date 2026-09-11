pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Trigger settings for the Automation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAutomationsRequestTriggerSettings {
    /// The type of Automation workflow.
    pub workflow_type: CreateAutomationsRequestTriggerSettingsWorkflowType,
}

impl CreateAutomationsRequestTriggerSettings {
    pub fn builder() -> CreateAutomationsRequestTriggerSettingsBuilder {
        <CreateAutomationsRequestTriggerSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAutomationsRequestTriggerSettingsBuilder {
    workflow_type: Option<CreateAutomationsRequestTriggerSettingsWorkflowType>,
}

impl CreateAutomationsRequestTriggerSettingsBuilder {
    pub fn workflow_type(mut self, value: CreateAutomationsRequestTriggerSettingsWorkflowType) -> Self {
        self.workflow_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAutomationsRequestTriggerSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_type`](CreateAutomationsRequestTriggerSettingsBuilder::workflow_type)
    pub fn build(self) -> Result<CreateAutomationsRequestTriggerSettings, BuildError> {
        Ok(CreateAutomationsRequestTriggerSettings {
            workflow_type: self.workflow_type.ok_or_else(|| BuildError::missing_field("workflow_type"))?,
        })
    }
}
