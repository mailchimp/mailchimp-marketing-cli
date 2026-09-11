pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The HTML and plain-text content for a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignContent {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<CampaignContentLinksItem>>,
    /// The Archive HTML for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_html: Option<String>,
    /// The raw HTML for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// The plain-text portion of the campaign. If left unspecified, we'll generate this automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_text: Option<String>,
    /// Content options for multivariate campaigns.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variate_contents: Option<Vec<CampaignContentVariateContentsItem>>,
}

impl CampaignContent {
    pub fn builder() -> CampaignContentBuilder {
        <CampaignContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignContentBuilder {
    links: Option<Vec<CampaignContentLinksItem>>,
    archive_html: Option<String>,
    html: Option<String>,
    plain_text: Option<String>,
    variate_contents: Option<Vec<CampaignContentVariateContentsItem>>,
}

impl CampaignContentBuilder {
    pub fn links(mut self, value: Vec<CampaignContentLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn archive_html(mut self, value: impl Into<String>) -> Self {
        self.archive_html = Some(value.into());
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

    pub fn variate_contents(mut self, value: Vec<CampaignContentVariateContentsItem>) -> Self {
        self.variate_contents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignContent`].
    pub fn build(self) -> Result<CampaignContent, BuildError> {
        Ok(CampaignContent {
            links: self.links,
            archive_html: self.archive_html,
            html: self.html,
            plain_text: self.plain_text,
            variate_contents: self.variate_contents,
        })
    }
}
