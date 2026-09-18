pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific product image.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListStoreProductImagesResponseImagesItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreProductImagesResponseImagesItemLinksItem>>,
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

impl ListStoreProductImagesResponseImagesItem {
    pub fn builder() -> ListStoreProductImagesResponseImagesItemBuilder {
        <ListStoreProductImagesResponseImagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreProductImagesResponseImagesItemBuilder {
    links: Option<Vec<ListStoreProductImagesResponseImagesItemLinksItem>>,
    id: Option<String>,
    url: Option<String>,
    variant_ids: Option<Vec<String>>,
}

impl ListStoreProductImagesResponseImagesItemBuilder {
    pub fn links(mut self, value: Vec<ListStoreProductImagesResponseImagesItemLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListStoreProductImagesResponseImagesItem`].
    pub fn build(self) -> Result<ListStoreProductImagesResponseImagesItem, BuildError> {
        Ok(ListStoreProductImagesResponseImagesItem {
            links: self.links,
            id: self.id,
            url: self.url,
            variant_ids: self.variant_ids,
        })
    }
}
