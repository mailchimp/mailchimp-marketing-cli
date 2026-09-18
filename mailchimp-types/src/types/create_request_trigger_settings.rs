pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Trigger settings for the Automation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateRequestTriggerSettings {
    /// The type of Automation workflow.
    pub workflow_type: CreateRequestTriggerSettingsWorkflowType,
}

impl CreateRequestTriggerSettings {
    pub fn builder() -> CreateRequestTriggerSettingsBuilder {
        <CreateRequestTriggerSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestTriggerSettingsBuilder {
    workflow_type: Option<CreateRequestTriggerSettingsWorkflowType>,
}

impl CreateRequestTriggerSettingsBuilder {
    pub fn workflow_type(mut self, value: CreateRequestTriggerSettingsWorkflowType) -> Self {
        self.workflow_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequestTriggerSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_type`](CreateRequestTriggerSettingsBuilder::workflow_type)
    pub fn build(self) -> Result<CreateRequestTriggerSettings, BuildError> {
        Ok(CreateRequestTriggerSettings {
            workflow_type: self.workflow_type.ok_or_else(|| BuildError::missing_field("workflow_type"))?,
        })
    }
}
