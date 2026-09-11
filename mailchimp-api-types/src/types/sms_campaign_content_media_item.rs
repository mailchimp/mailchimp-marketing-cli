pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SmsCampaignContentMediaItem {
    /// The URL of the media file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SmsCampaignContentMediaItem {
    pub fn builder() -> SmsCampaignContentMediaItemBuilder {
        <SmsCampaignContentMediaItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SmsCampaignContentMediaItemBuilder {
    url: Option<String>,
}

impl SmsCampaignContentMediaItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SmsCampaignContentMediaItem`].
    pub fn build(self) -> Result<SmsCampaignContentMediaItem, BuildError> {
        Ok(SmsCampaignContentMediaItem {
            url: self.url,
        })
    }
}
