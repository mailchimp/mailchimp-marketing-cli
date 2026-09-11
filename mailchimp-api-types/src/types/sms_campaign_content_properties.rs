pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Additional content properties.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SmsCampaignContentProperties {
    /// The content type of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// The sender identifier for the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// The language of the opt-out message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optout_message_language: Option<String>,
}

impl SmsCampaignContentProperties {
    pub fn builder() -> SmsCampaignContentPropertiesBuilder {
        <SmsCampaignContentPropertiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SmsCampaignContentPropertiesBuilder {
    content_type: Option<String>,
    sender: Option<String>,
    optout_message_language: Option<String>,
}

impl SmsCampaignContentPropertiesBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn sender(mut self, value: impl Into<String>) -> Self {
        self.sender = Some(value.into());
        self
    }

    pub fn optout_message_language(mut self, value: impl Into<String>) -> Self {
        self.optout_message_language = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SmsCampaignContentProperties`].
    pub fn build(self) -> Result<SmsCampaignContentProperties, BuildError> {
        Ok(SmsCampaignContentProperties {
            content_type: self.content_type,
            sender: self.sender,
            optout_message_language: self.optout_message_language,
        })
    }
}
