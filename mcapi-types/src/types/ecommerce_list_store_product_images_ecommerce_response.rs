pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of a product's images.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListStoreProductImagesEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreProductImagesEcommerceResponseLinksItem>>,
    /// An array of objects, each representing a product image resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<ListStoreProductImagesEcommerceResponseImagesItem>>,
    /// The product id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStoreProductImagesEcommerceResponse {
    pub fn builder() -> ListStoreProductImagesEcommerceResponseBuilder {
        <ListStoreProductImagesEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreProductImagesEcommerceResponseBuilder {
    links: Option<Vec<ListStoreProductImagesEcommerceResponseLinksItem>>,
    images: Option<Vec<ListStoreProductImagesEcommerceResponseImagesItem>>,
    product_id: Option<String>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreProductImagesEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreProductImagesEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn images(mut self, value: Vec<ListStoreProductImagesEcommerceResponseImagesItem>) -> Self {
        self.images = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStoreProductImagesEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreProductImagesEcommerceResponse, BuildError> {
        Ok(ListStoreProductImagesEcommerceResponse {
            links: self.links,
            images: self.images,
            product_id: self.product_id,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
