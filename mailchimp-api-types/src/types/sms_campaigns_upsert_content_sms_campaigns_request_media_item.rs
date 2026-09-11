pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertContentSmsCampaignsRequestMediaItem {
    /// The URL of the media file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpsertContentSmsCampaignsRequestMediaItem {
    pub fn builder() -> UpsertContentSmsCampaignsRequestMediaItemBuilder {
        <UpsertContentSmsCampaignsRequestMediaItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertContentSmsCampaignsRequestMediaItemBuilder {
    url: Option<String>,
}

impl UpsertContentSmsCampaignsRequestMediaItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpsertContentSmsCampaignsRequestMediaItem`].
    pub fn build(self) -> Result<UpsertContentSmsCampaignsRequestMediaItem, BuildError> {
        Ok(UpsertContentSmsCampaignsRequestMediaItem {
            url: self.url,
        })
    }
}
