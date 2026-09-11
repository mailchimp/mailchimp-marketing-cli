pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of a store's carts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreCartsEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreCartsEcommerceResponseLinksItem>>,
    /// An array of objects, each representing a cart.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carts: Option<Vec<ECommerceCart>>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStoreCartsEcommerceResponse {
    pub fn builder() -> ListStoreCartsEcommerceResponseBuilder {
        <ListStoreCartsEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreCartsEcommerceResponseBuilder {
    links: Option<Vec<ListStoreCartsEcommerceResponseLinksItem>>,
    carts: Option<Vec<ECommerceCart>>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreCartsEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreCartsEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn carts(mut self, value: Vec<ECommerceCart>) -> Self {
        self.carts = Some(value);
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

    /// Consumes the builder and constructs a [`ListStoreCartsEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreCartsEcommerceResponse, BuildError> {
        Ok(ListStoreCartsEcommerceResponse {
            links: self.links,
            carts: self.carts,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
