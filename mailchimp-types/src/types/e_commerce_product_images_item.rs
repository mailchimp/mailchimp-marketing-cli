pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific product image.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommerceProductImagesItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceProductImagesItemLinksItem>>,
    /// A unique identifier for the product image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The URL for a product image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The list of product variants using the image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_ids: Option<Vec<String>>,
}

impl ECommerceProductImagesItem {
    pub fn builder() -> ECommerceProductImagesItemBuilder {
        <ECommerceProductImagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceProductImagesItemBuilder {
    links: Option<Vec<ECommerceProductImagesItemLinksItem>>,
    id: Option<String>,
    url: Option<String>,
    variant_ids: Option<Vec<String>>,
}

impl ECommerceProductImagesItemBuilder {
    pub fn links(mut self, value: Vec<ECommerceProductImagesItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn variant_ids(mut self, value: Vec<String>) -> Self {
        self.variant_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceProductImagesItem`].
    pub fn build(self) -> Result<ECommerceProductImagesItem, BuildError> {
        Ok(ECommerceProductImagesItem {
            links: self.links,
            id: self.id,
            url: self.url,
            variant_ids: self.variant_ids,
        })
    }
}
