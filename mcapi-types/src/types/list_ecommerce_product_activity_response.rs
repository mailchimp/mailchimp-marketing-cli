pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of ecommerce products.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEcommerceProductActivityResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListEcommerceProductActivityResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<ListEcommerceProductActivityResponseProductsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListEcommerceProductActivityResponse {
    pub fn builder() -> ListEcommerceProductActivityResponseBuilder {
        <ListEcommerceProductActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEcommerceProductActivityResponseBuilder {
    links: Option<Vec<ListEcommerceProductActivityResponseLinksItem>>,
    products: Option<Vec<ListEcommerceProductActivityResponseProductsItem>>,
    total_items: Option<i64>,
}

impl ListEcommerceProductActivityResponseBuilder {
    pub fn links(mut self, value: Vec<ListEcommerceProductActivityResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<ListEcommerceProductActivityResponseProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEcommerceProductActivityResponse`].
    pub fn build(self) -> Result<ListEcommerceProductActivityResponse, BuildError> {
        Ok(ListEcommerceProductActivityResponse {
            links: self.links,
            products: self.products,
            total_items: self.total_items,
        })
    }
}
