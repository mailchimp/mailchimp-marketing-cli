pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of orders in a store.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreOrdersResponse {
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
    pub links: Option<Vec<ListStoreOrdersResponseLinksItem>>,
}

impl ListStoreOrdersResponse {
    pub fn builder() -> ListStoreOrdersResponseBuilder {
        <ListStoreOrdersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreOrdersResponseBuilder {
    store_id: Option<String>,
    orders: Option<Vec<ECommerceOrder>>,
    total_items: Option<i64>,
    links: Option<Vec<ListStoreOrdersResponseLinksItem>>,
}

impl ListStoreOrdersResponseBuilder {
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

    pub fn links(mut self, value: Vec<ListStoreOrdersResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStoreOrdersResponse`].
    pub fn build(self) -> Result<ListStoreOrdersResponse, BuildError> {
        Ok(ListStoreOrdersResponse {
            store_id: self.store_id,
            orders: self.orders,
            total_items: self.total_items,
            links: self.links,
        })
    }
}
