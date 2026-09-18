pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of an order's line items.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreOrderLinesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreOrderLinesResponseLinksItem>>,
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

impl ListStoreOrderLinesResponse {
    pub fn builder() -> ListStoreOrderLinesResponseBuilder {
        <ListStoreOrderLinesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreOrderLinesResponseBuilder {
    links: Option<Vec<ListStoreOrderLinesResponseLinksItem>>,
    lines: Option<Vec<ECommerceOrderLineItem>>,
    order_id: Option<String>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreOrderLinesResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreOrderLinesResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListStoreOrderLinesResponse`].
    pub fn build(self) -> Result<ListStoreOrderLinesResponse, BuildError> {
        Ok(ListStoreOrderLinesResponse {
            links: self.links,
            lines: self.lines,
            order_id: self.order_id,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
