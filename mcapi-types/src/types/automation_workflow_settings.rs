pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The settings for the Automation workflow.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowSettings {
    /// Whether Mailchimp [authenticated](https://mailchimp.com/help/about-email-authentication/) the Automation. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authenticate: Option<bool>,
    /// Whether to automatically append Mailchimp's [default footer](https://mailchimp.com/help/about-campaign-footers/) to the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_footer: Option<bool>,
    /// The 'from' name for the Automation (not an email address).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// Whether to automatically inline the CSS included with the Automation content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_css: Option<bool>,
    /// The reply-to email address for the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to: Option<String>,
    /// The title of the Automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The Automation's custom 'To' name, typically the first name [audience field](https://mailchimp.com/help/getting-started-with-merge-tags/).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_name: Option<String>,
    /// Whether to use Mailchimp Conversation feature to manage replies
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_conversation: Option<bool>,
}

impl AutomationWorkflowSettings {
    pub fn builder() -> AutomationWorkflowSettingsBuilder {
        <AutomationWorkflowSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowSettingsBuilder {
    authenticate: Option<bool>,
    auto_footer: Option<bool>,
    from_name: Option<String>,
    inline_css: Option<bool>,
    reply_to: Option<String>,
    title: Option<String>,
    to_name: Option<String>,
    use_conversation: Option<bool>,
}

impl AutomationWorkflowSettingsBuilder {
    pub fn authenticate(mut self, value: bool) -> Self {
        self.authenticate = Some(value);
        self
    }

    pub fn auto_footer(mut self, value: bool) -> Self {
        self.auto_footer = Some(value);
        self
    }

    pub fn from_name(mut self, value: impl Into<String>) -> Self {
        self.from_name = Some(value.into());
        self
    }

    pub fn inline_css(mut self, value: bool) -> Self {
        self.inline_css = Some(value);
        self
    }

    pub fn reply_to(mut self, value: impl Into<String>) -> Self {
        self.reply_to = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn to_name(mut self, value: impl Into<String>) -> Self {
        self.to_name = Some(value.into());
        self
    }

    pub fn use_conversation(mut self, value: bool) -> Self {
        self.use_conversation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowSettings`].
    pub fn build(self) -> Result<AutomationWorkflowSettings, BuildError> {
        Ok(AutomationWorkflowSettings {
            authenticate: self.authenticate,
            auto_footer: self.auto_footer,
            from_name: self.from_name,
            inline_css: self.inline_css,
            reply_to: self.reply_to,
            title: self.title,
            to_name: self.to_name,
            use_conversation: self.use_conversation,
        })
    }
}
