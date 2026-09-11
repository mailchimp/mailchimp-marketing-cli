pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of an order's line items.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreOrderLinesEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreOrderLinesEcommerceResponseLinksItem>>,
    /// An array of objects, each representing an order's line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<ECommerceOrderLineItem>>,
    /// The order id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStoreOrderLinesEcommerceResponse {
    pub fn builder() -> ListStoreOrderLinesEcommerceResponseBuilder {
        <ListStoreOrderLinesEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreOrderLinesEcommerceResponseBuilder {
    links: Option<Vec<ListStoreOrderLinesEcommerceResponseLinksItem>>,
    lines: Option<Vec<ECommerceOrderLineItem>>,
    order_id: Option<String>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreOrderLinesEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreOrderLinesEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn lines(mut self, value: Vec<ECommerceOrderLineItem>) -> Self {
        self.lines = Some(value);
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`ListStoreOrderLinesEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreOrderLinesEcommerceResponse, BuildError> {
        Ok(ListStoreOrderLinesEcommerceResponse {
            links: self.links,
            lines: self.lines,
            order_id: self.order_id,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
