pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignContentVariateContentsItem {
    /// Label used to identify the content option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_label: Option<String>,
    /// The raw HTML for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// The plain-text portion of the campaign. If left unspecified, we'll generate this automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
}

impl CampaignContentVariateContentsItem {
    pub fn builder() -> CampaignContentVariateContentsItemBuilder {
        <CampaignContentVariateContentsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignContentVariateContentsItemBuilder {
    content_label: Option<String>,
    html: Option<String>,
    plain_text: Option<String>,
}

impl CampaignContentVariateContentsItemBuilder {
    pub fn content_label(mut self, value: impl Into<String>) -> Self {
        self.content_label = Some(value.into());
        self
    }

    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn plain_text(mut self, value: impl Into<String>) -> Self {
        self.plain_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignContentVariateContentsItem`].
    pub fn build(self) -> Result<CampaignContentVariateContentsItem, BuildError> {
        Ok(CampaignContentVariateContentsItem {
            content_label: self.content_label,
            html: self.html,
            plain_text: self.plain_text,
        })
    }
}
