pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The delay settings for an Automation email.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowEmailDelay {
    /// The action that triggers the delay of an Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<AutomationWorkflowEmailDelayAction>,
    /// The user-friendly description of the action that triggers an Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_description: Option<String>,
    /// The delay amount for an Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Whether the delay settings describe before or after the delay action of an Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<AutomationWorkflowEmailDelayDirection>,
    /// The user-friendly description of the delay and trigger action settings for an Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_description: Option<String>,
    /// The type of delay for an Automation email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AutomationWorkflowEmailDelayType>,
}

impl AutomationWorkflowEmailDelay {
    pub fn builder() -> AutomationWorkflowEmailDelayBuilder {
        <AutomationWorkflowEmailDelayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailDelayBuilder {
    action: Option<AutomationWorkflowEmailDelayAction>,
    action_description: Option<String>,
    amount: Option<i64>,
    direction: Option<AutomationWorkflowEmailDelayDirection>,
    full_description: Option<String>,
    r#type: Option<AutomationWorkflowEmailDelayType>,
}

impl AutomationWorkflowEmailDelayBuilder {
    pub fn action(mut self, value: AutomationWorkflowEmailDelayAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn action_description(mut self, value: impl Into<String>) -> Self {
        self.action_description = Some(value.into());
        self
    }

    pub fn amount(mut self, value: i64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn direction(mut self, value: AutomationWorkflowEmailDelayDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn full_description(mut self, value: impl Into<String>) -> Self {
        self.full_description = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: AutomationWorkflowEmailDelayType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmailDelay`].
    pub fn build(self) -> Result<AutomationWorkflowEmailDelay, BuildError> {
        Ok(AutomationWorkflowEmailDelay {
            action: self.action,
            action_description: self.action_description,
            amount: self.amount,
            direction: self.direction,
            full_description: self.full_description,
            r#type: self.r#type,
        })
    }
}
