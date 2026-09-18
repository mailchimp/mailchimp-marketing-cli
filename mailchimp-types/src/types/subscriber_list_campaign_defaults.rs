pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SubscriberListCampaignDefaults {
    /// The default from email for campaigns sent to this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email: Option<String>,
    /// The default from name for campaigns sent to this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_name: Option<String>,
    /// The default language for this lists's forms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// The default subject line for campaigns sent to this list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

impl SubscriberListCampaignDefaults {
    pub fn builder() -> SubscriberListCampaignDefaultsBuilder {
        <SubscriberListCampaignDefaultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriberListCampaignDefaultsBuilder {
    from_email: Option<String>,
    from_name: Option<String>,
    language: Option<String>,
    subject: Option<String>,
}

impl SubscriberListCampaignDefaultsBuilder {
    pub fn from_email(mut self, value: impl Into<String>) -> Self {
        self.from_email = Some(value.into());
        self
    }

    pub fn from_name(mut self, value: impl Into<String>) -> Self {
        self.from_name = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriberListCampaignDefaults`].
    pub fn build(self) -> Result<SubscriberListCampaignDefaults, BuildError> {
        Ok(SubscriberListCampaignDefaults {
            from_email: self.from_email,
            from_name: self.from_name,
            language: self.language,
            subject: self.subject,
        })
    }
}
