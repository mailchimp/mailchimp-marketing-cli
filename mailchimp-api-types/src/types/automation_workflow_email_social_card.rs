pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The preview for the campaign, rendered by social networks like Facebook and Twitter. [Learn more](https://mailchimp.com/help/enable-and-customize-social-cards/).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutomationWorkflowEmailSocialCard {
    /// A short summary of the campaign to display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The url for the header image for the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The title for the card. Typically the subject line of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl AutomationWorkflowEmailSocialCard {
    pub fn builder() -> AutomationWorkflowEmailSocialCardBuilder {
        <AutomationWorkflowEmailSocialCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationWorkflowEmailSocialCardBuilder {
    description: Option<String>,
    image_url: Option<String>,
    title: Option<String>,
}

impl AutomationWorkflowEmailSocialCardBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationWorkflowEmailSocialCard`].
    pub fn build(self) -> Result<AutomationWorkflowEmailSocialCard, BuildError> {
        Ok(AutomationWorkflowEmailSocialCard {
            description: self.description,
            image_url: self.image_url,
            title: self.title,
        })
    }
}
