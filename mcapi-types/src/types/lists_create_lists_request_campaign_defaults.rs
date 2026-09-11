pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// [Default values for campaigns](https://mailchimp.com/help/edit-your-emails-subject-preview-text-from-name-or-from-email-address/) created for this list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateListsRequestCampaignDefaults {
    /// The default from email for campaigns sent to this list.
    #[serde(default)]
    pub from_email: String,
    /// The default from name for campaigns sent to this list.
    #[serde(default)]
    pub from_name: String,
    /// The default language for this lists's forms.
    #[serde(default)]
    pub language: String,
    /// The default subject line for campaigns sent to this list.
    #[serde(default)]
    pub subject: String,
}

impl CreateListsRequestCampaignDefaults {
    pub fn builder() -> CreateListsRequestCampaignDefaultsBuilder {
        <CreateListsRequestCampaignDefaultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateListsRequestCampaignDefaultsBuilder {
    from_email: Option<String>,
    from_name: Option<String>,
    language: Option<String>,
    subject: Option<String>,
}

impl CreateListsRequestCampaignDefaultsBuilder {
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

    /// Consumes the builder and constructs a [`CreateListsRequestCampaignDefaults`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_email`](CreateListsRequestCampaignDefaultsBuilder::from_email)
    /// - [`from_name`](CreateListsRequestCampaignDefaultsBuilder::from_name)
    /// - [`language`](CreateListsRequestCampaignDefaultsBuilder::language)
    /// - [`subject`](CreateListsRequestCampaignDefaultsBuilder::subject)
    pub fn build(self) -> Result<CreateListsRequestCampaignDefaults, BuildError> {
        Ok(CreateListsRequestCampaignDefaults {
            from_email: self.from_email.ok_or_else(|| BuildError::missing_field("from_email"))?,
            from_name: self.from_name.ok_or_else(|| BuildError::missing_field("from_name"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            subject: self.subject.ok_or_else(|| BuildError::missing_field("subject"))?,
        })
    }
}
