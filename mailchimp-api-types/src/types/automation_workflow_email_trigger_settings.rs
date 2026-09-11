pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Available triggers for Automation workflows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowEmailTriggerSettings {
    /// A workflow's runtime settings for an Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AutomationWorkflowEmailTriggerSettingsRuntime>,
    /// The number of emails in the Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_emails_count: Option<i64>,
    /// The title of the workflow type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_title: Option<String>,
    /// The type of Automation workflow.
    pub workflow_type: AutomationWorkflowEmailTriggerSettingsWorkflowType,
}

impl AutomationWorkflowEmailTriggerSettings {
    pub fn builder() -> AutomationWorkflowEmailTriggerSettingsBuilder {
        <AutomationWorkflowEmailTriggerSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailTriggerSettingsBuilder {
    runtime: Option<AutomationWorkflowEmailTriggerSettingsRuntime>,
    workflow_emails_count: Option<i64>,
    workflow_title: Option<String>,
    workflow_type: Option<AutomationWorkflowEmailTriggerSettingsWorkflowType>,
}

impl AutomationWorkflowEmailTriggerSettingsBuilder {
    pub fn runtime(mut self, value: AutomationWorkflowEmailTriggerSettingsRuntime) -> Self {
        self.runtime = Some(value);
        self
    }

    pub fn workflow_emails_count(mut self, value: i64) -> Self {
        self.workflow_emails_count = Some(value);
        self
    }

    pub fn workflow_title(mut self, value: impl Into<String>) -> Self {
        self.workflow_title = Some(value.into());
        self
    }

    pub fn workflow_type(mut self, value: AutomationWorkflowEmailTriggerSettingsWorkflowType) -> Self {
        self.workflow_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmailTriggerSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_type`](AutomationWorkflowEmailTriggerSettingsBuilder::workflow_type)
    pub fn build(self) -> Result<AutomationWorkflowEmailTriggerSettings, BuildError> {
        Ok(AutomationWorkflowEmailTriggerSettings {
            runtime: self.runtime,
            workflow_emails_count: self.workflow_emails_count,
            workflow_title: self.workflow_title,
            workflow_type: self.workflow_type.ok_or_else(|| BuildError::missing_field("workflow_type"))?,
        })
    }
}
