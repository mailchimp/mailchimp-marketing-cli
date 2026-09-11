pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific product image.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EcommerceStoresOrdersPostImagesItem {
    /// A unique identifier for the product image.
    #[serde(default)]
    pub id: String,
    /// The URL for a product image.
    #[serde(default)]
    pub url: String,
    /// The list of product variants using the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_ids: Option<Vec<EcommerceStoresOrdersPostImagesItemVariantIdsItem>>,
}

impl EcommerceStoresOrdersPostImagesItem {
    pub fn builder() -> EcommerceStoresOrdersPostImagesItemBuilder {
        <EcommerceStoresOrdersPostImagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EcommerceStoresOrdersPostImagesItemBuilder {
    id: Option<String>,
    url: Option<String>,
    variant_ids: Option<Vec<EcommerceStoresOrdersPostImagesItemVariantIdsItem>>,
}

impl EcommerceStoresOrdersPostImagesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn variant_ids(mut self, value: Vec<EcommerceStoresOrdersPostImagesItemVariantIdsItem>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EcommerceStoresOrdersPostImagesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EcommerceStoresOrdersPostImagesItemBuilder::id)
    /// - [`url`](EcommerceStoresOrdersPostImagesItemBuilder::url)
    pub fn build(self) -> Result<EcommerceStoresOrdersPostImagesItem, BuildError> {
        Ok(EcommerceStoresOrdersPostImagesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            variant_ids: self.variant_ids,
        })
    }
}
