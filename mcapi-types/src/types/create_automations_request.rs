pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAutomationsRequest {
    /// List settings for the Automation.
    #[serde(default)]
    pub recipients: CreateAutomationsRequestRecipients,
    /// The settings for the Automation workflow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateAutomationsRequestSettings>,
    /// Trigger settings for the Automation.
    pub trigger_settings: CreateAutomationsRequestTriggerSettings,
}

impl CreateAutomationsRequest {
    pub fn builder() -> CreateAutomationsRequestBuilder {
        <CreateAutomationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAutomationsRequestBuilder {
    recipients: Option<CreateAutomationsRequestRecipients>,
    settings: Option<CreateAutomationsRequestSettings>,
    trigger_settings: Option<CreateAutomationsRequestTriggerSettings>,
}

impl CreateAutomationsRequestBuilder {
    pub fn recipients(mut self, value: CreateAutomationsRequestRecipients) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn settings(mut self, value: CreateAutomationsRequestSettings) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn trigger_settings(mut self, value: CreateAutomationsRequestTriggerSettings) -> Self {
        self.trigger_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAutomationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`recipients`](CreateAutomationsRequestBuilder::recipients)
    /// - [`trigger_settings`](CreateAutomationsRequestBuilder::trigger_settings)
    pub fn build(self) -> Result<CreateAutomationsRequest, BuildError> {
        Ok(CreateAutomationsRequest {
            recipients: self.recipients.ok_or_else(|| BuildError::missing_field("recipients"))?,
            settings: self.settings,
            trigger_settings: self.trigger_settings.ok_or_else(|| BuildError::missing_field("trigger_settings"))?,
        })
    }
}

