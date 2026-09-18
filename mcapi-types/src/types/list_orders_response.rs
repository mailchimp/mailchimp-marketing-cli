pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of orders in an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOrdersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListOrdersResponseLinksItem>>,
    /// An array of objects, each representing an order resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<ECommerceOrder>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListOrdersResponse {
    pub fn builder() -> ListOrdersResponseBuilder {
        <ListOrdersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersResponseBuilder {
    links: Option<Vec<ListOrdersResponseLinksItem>>,
    orders: Option<Vec<ECommerceOrder>>,
    total_items: Option<i64>,
}

impl ListOrdersResponseBuilder {
    pub fn links(mut self, value: Vec<ListOrdersResponseLinksItem>) -> Self {
        self.links = Some(value);
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

    /// Consumes the builder and constructs a [`ListOrdersResponse`].
    pub fn build(self) -> Result<ListOrdersResponse, BuildError> {
        Ok(ListOrdersResponse {
            links: self.links,
            orders: self.orders,
            total_items: self.total_items,
        })
    }
}
