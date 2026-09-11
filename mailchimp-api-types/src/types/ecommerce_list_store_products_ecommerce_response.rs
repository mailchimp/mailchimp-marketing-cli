pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of a store's products.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreProductsEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreProductsEcommerceResponseLinksItem>>,
    /// An array of objects, each representing a store product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<ECommerceProduct>>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStoreProductsEcommerceResponse {
    pub fn builder() -> ListStoreProductsEcommerceResponseBuilder {
        <ListStoreProductsEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreProductsEcommerceResponseBuilder {
    links: Option<Vec<ListStoreProductsEcommerceResponseLinksItem>>,
    products: Option<Vec<ECommerceProduct>>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreProductsEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreProductsEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<ECommerceProduct>) -> Self {
        self.products = Some(value);
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

    /// Consumes the builder and constructs a [`ListStoreProductsEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreProductsEcommerceResponse, BuildError> {
        Ok(ListStoreProductsEcommerceResponse {
            links: self.links,
            products: self.products,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
