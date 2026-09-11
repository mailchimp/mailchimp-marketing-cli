pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of orders in an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListOrdersEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListOrdersEcommerceResponseLinksItem>>,
    /// An array of objects, each representing an order resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<ECommerceOrder>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListOrdersEcommerceResponse {
    pub fn builder() -> ListOrdersEcommerceResponseBuilder {
        <ListOrdersEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrdersEcommerceResponseBuilder {
    links: Option<Vec<ListOrdersEcommerceResponseLinksItem>>,
    orders: Option<Vec<ECommerceOrder>>,
    total_items: Option<i64>,
}

impl ListOrdersEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListOrdersEcommerceResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListOrdersEcommerceResponse`].
    pub fn build(self) -> Result<ListOrdersEcommerceResponse, BuildError> {
        Ok(ListOrdersEcommerceResponse {
            links: self.links,
            orders: self.orders,
            total_items: self.total_items,
        })
    }
}
