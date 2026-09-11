pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Settings for the campaign including the email subject, from name, and from email address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEmailAutomationsRequestSettings {
    /// The 'from' name for the Automation (not an email address).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// The preview text for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    /// The reply-to email address for the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// The subject line for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_line: Option<String>,
    /// The title of the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateEmailAutomationsRequestSettings {
    pub fn builder() -> UpdateEmailAutomationsRequestSettingsBuilder {
        <UpdateEmailAutomationsRequestSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEmailAutomationsRequestSettingsBuilder {
    from_name: Option<String>,
    preview_text: Option<String>,
    reply_to: Option<String>,
    subject_line: Option<String>,
    title: Option<String>,
}

impl UpdateEmailAutomationsRequestSettingsBuilder {
    pub fn from_name(mut self, value: impl Into<String>) -> Self {
        self.from_name = Some(value.into());
        self
    }

    pub fn preview_text(mut self, value: impl Into<String>) -> Self {
        self.preview_text = Some(value.into());
        self
    }

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }

    pub fn subject_line(mut self, value: impl Into<String>) -> Self {
        self.subject_line = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateEmailAutomationsRequestSettings`].
    pub fn build(self) -> Result<UpdateEmailAutomationsRequestSettings, BuildError> {
        Ok(UpdateEmailAutomationsRequestSettings {
            from_name: self.from_name,
            preview_text: self.preview_text,
            reply_to: self.reply_to,
            subject_line: self.subject_line,
            title: self.title,
        })
    }
}
