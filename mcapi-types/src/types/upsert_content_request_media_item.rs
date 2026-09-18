pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertContentRequestMediaItem {
    /// The URL of the media file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl UpsertContentRequestMediaItem {
    pub fn builder() -> UpsertContentRequestMediaItemBuilder {
        <UpsertContentRequestMediaItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertContentRequestMediaItemBuilder {
    url: Option<String>,
}

impl UpsertContentRequestMediaItemBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpsertContentRequestMediaItem`].
    pub fn build(self) -> Result<UpsertContentRequestMediaItem, BuildError> {
        Ok(UpsertContentRequestMediaItem {
            url: self.url,
        })
    }
}
