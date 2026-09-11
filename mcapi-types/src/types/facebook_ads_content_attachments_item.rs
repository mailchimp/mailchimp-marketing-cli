pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FacebookAdsContentAttachmentsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FacebookAdsContentAttachmentsItem {
    pub fn builder() -> FacebookAdsContentAttachmentsItemBuilder {
        <FacebookAdsContentAttachmentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsContentAttachmentsItemBuilder {
    call_to_action: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    link_url: Option<String>,
    name: Option<String>,
}

impl FacebookAdsContentAttachmentsItemBuilder {
    pub fn call_to_action(mut self, value: impl Into<String>) -> Self {
        self.call_to_action = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn link_url(mut self, value: impl Into<String>) -> Self {
        self.link_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsContentAttachmentsItem`].
    pub fn build(self) -> Result<FacebookAdsContentAttachmentsItem, BuildError> {
        Ok(FacebookAdsContentAttachmentsItem {
            call_to_action: self.call_to_action,
            description: self.description,
            image_url: self.image_url,
            link_url: self.link_url,
            name: self.name,
        })
    }
}
