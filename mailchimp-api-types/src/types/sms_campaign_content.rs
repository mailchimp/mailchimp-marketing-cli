pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The content of an SMS campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SmsCampaignContent {
    /// The SMS message body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_body: Option<String>,
    /// The estimated number of message segments this content will use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_segments: Option<i64>,
    /// The merge fields used in the message body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<Vec<String>>,
    /// Attached images or files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<SmsCampaignContentMediaItem>>,
    /// The source that created or imported this content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SmsCampaignContentSource>,
    /// Additional content properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SmsCampaignContentProperties>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<SmsCampaignContentLinksItem>>,
}

impl SmsCampaignContent {
    pub fn builder() -> SmsCampaignContentBuilder {
        <SmsCampaignContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SmsCampaignContentBuilder {
    message_body: Option<String>,
    estimated_segments: Option<i64>,
    merge_fields: Option<Vec<String>>,
    media: Option<Vec<SmsCampaignContentMediaItem>>,
    source: Option<SmsCampaignContentSource>,
    properties: Option<SmsCampaignContentProperties>,
    links: Option<Vec<SmsCampaignContentLinksItem>>,
}

impl SmsCampaignContentBuilder {
    pub fn message_body(mut self, value: impl Into<String>) -> Self {
        self.message_body = Some(value.into());
        self
    }

    pub fn estimated_segments(mut self, value: i64) -> Self {
        self.estimated_segments = Some(value);
        self
    }

    pub fn merge_fields(mut self, value: Vec<String>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn media(mut self, value: Vec<SmsCampaignContentMediaItem>) -> Self {
        self.media = Some(value);
        self
    }

    pub fn source(mut self, value: SmsCampaignContentSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn properties(mut self, value: SmsCampaignContentProperties) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<SmsCampaignContentLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SmsCampaignContent`].
    pub fn build(self) -> Result<SmsCampaignContent, BuildError> {
        Ok(SmsCampaignContent {
            message_body: self.message_body,
            estimated_segments: self.estimated_segments,
            merge_fields: self.merge_fields,
            media: self.media,
            source: self.source,
            properties: self.properties,
            links: self.links,
        })
    }
}
