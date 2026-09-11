pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The settings for the Automation workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAutomationsRequestSettings {
    /// The 'from' name for the Automation (not an email address).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// The reply-to email address for the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
}

impl CreateAutomationsRequestSettings {
    pub fn builder() -> CreateAutomationsRequestSettingsBuilder {
        <CreateAutomationsRequestSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAutomationsRequestSettingsBuilder {
    from_name: Option<String>,
    reply_to: Option<String>,
}

impl CreateAutomationsRequestSettingsBuilder {
    pub fn from_name(mut self, value: impl Into<String>) -> Self {
        self.from_name = Some(value.into());
        self
    }

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAutomationsRequestSettings`].
    pub fn build(self) -> Result<CreateAutomationsRequestSettings, BuildError> {
        Ok(CreateAutomationsRequestSettings {
            from_name: self.from_name,
            reply_to: self.reply_to,
        })
    }
}
