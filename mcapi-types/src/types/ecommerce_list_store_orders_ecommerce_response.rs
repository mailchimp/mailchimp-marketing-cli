pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of orders in a store.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreOrdersEcommerceResponse {
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// An array of objects, each representing an order in a store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<ECommerceOrder>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreOrdersEcommerceResponseLinksItem>>,
}

impl ListStoreOrdersEcommerceResponse {
    pub fn builder() -> ListStoreOrdersEcommerceResponseBuilder {
        <ListStoreOrdersEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreOrdersEcommerceResponseBuilder {
    store_id: Option<String>,
    orders: Option<Vec<ECommerceOrder>>,
    total_items: Option<i64>,
    links: Option<Vec<ListStoreOrdersEcommerceResponseLinksItem>>,
}

impl ListStoreOrdersEcommerceResponseBuilder {
    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    pub fn orders(mut self, value: Vec<ECommerceOrder>) -> Self {
        self.orders = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    pub fn links(mut self, value: Vec<ListStoreOrdersEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStoreOrdersEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreOrdersEcommerceResponse, BuildError> {
        Ok(ListStoreOrdersEcommerceResponse {
            store_id: self.store_id,
            orders: self.orders,
            total_items: self.total_items,
            links: self.links,
        })
    }
}
