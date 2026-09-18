pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific product image.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertStoreProductRequestImagesItem {
    /// A unique identifier for the product image.
    #[serde(default)]
    pub id: String,
    /// The URL for a product image.
    #[serde(default)]
    pub url: String,
    /// The list of product variants using the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_ids: Option<Vec<UpsertStoreProductRequestImagesItemVariantIdsItem>>,
}

impl UpsertStoreProductRequestImagesItem {
    pub fn builder() -> UpsertStoreProductRequestImagesItemBuilder {
        <UpsertStoreProductRequestImagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertStoreProductRequestImagesItemBuilder {
    id: Option<String>,
    url: Option<String>,
    variant_ids: Option<Vec<UpsertStoreProductRequestImagesItemVariantIdsItem>>,
}

impl UpsertStoreProductRequestImagesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn variant_ids(mut self, value: Vec<UpsertStoreProductRequestImagesItemVariantIdsItem>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertStoreProductRequestImagesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpsertStoreProductRequestImagesItemBuilder::id)
    /// - [`url`](UpsertStoreProductRequestImagesItemBuilder::url)
    pub fn build(self) -> Result<UpsertStoreProductRequestImagesItem, BuildError> {
        Ok(UpsertStoreProductRequestImagesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            variant_ids: self.variant_ids,
        })
    }
}
