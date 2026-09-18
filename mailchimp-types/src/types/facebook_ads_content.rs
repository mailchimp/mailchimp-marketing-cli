pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FacebookAdsContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<FacebookAdsContentAttachmentsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_to_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl FacebookAdsContent {
    pub fn builder() -> FacebookAdsContentBuilder {
        <FacebookAdsContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsContentBuilder {
    attachments: Option<Vec<FacebookAdsContentAttachmentsItem>>,
    call_to_action: Option<String>,
    description: Option<String>,
    image_url: Option<String>,
    link_url: Option<String>,
    message: Option<String>,
    title: Option<String>,
}

impl FacebookAdsContentBuilder {
    pub fn attachments(mut self, value: Vec<FacebookAdsContentAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

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

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsContent`].
    pub fn build(self) -> Result<FacebookAdsContent, BuildError> {
        Ok(FacebookAdsContent {
            attachments: self.attachments,
            call_to_action: self.call_to_action,
            description: self.description,
            image_url: self.image_url,
            link_url: self.link_url,
            message: self.message,
            title: self.title,
        })
    }
}
