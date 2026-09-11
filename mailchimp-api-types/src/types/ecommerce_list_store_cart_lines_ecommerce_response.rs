pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of a cart's line items.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreCartLinesEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreCartLinesEcommerceResponseLinksItem>>,
    /// The cart id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart_id: Option<String>,
    /// An array of objects, each representing a cart's line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<ECommerceCartLineItem>>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStoreCartLinesEcommerceResponse {
    pub fn builder() -> ListStoreCartLinesEcommerceResponseBuilder {
        <ListStoreCartLinesEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreCartLinesEcommerceResponseBuilder {
    links: Option<Vec<ListStoreCartLinesEcommerceResponseLinksItem>>,
    cart_id: Option<String>,
    lines: Option<Vec<ECommerceCartLineItem>>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreCartLinesEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreCartLinesEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn cart_id(mut self, value: impl Into<String>) -> Self {
        self.cart_id = Some(value.into());
        self
    }

    pub fn lines(mut self, value: Vec<ECommerceCartLineItem>) -> Self {
        self.lines = Some(value);
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

    /// Consumes the builder and constructs a [`ListStoreCartLinesEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreCartLinesEcommerceResponse, BuildError> {
        Ok(ListStoreCartLinesEcommerceResponse {
            links: self.links,
            cart_id: self.cart_id,
            lines: self.lines,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
