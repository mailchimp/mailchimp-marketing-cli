pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of ecommerce products.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEcommerceProductActivityReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListEcommerceProductActivityReportsResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<ListEcommerceProductActivityReportsResponseProductsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListEcommerceProductActivityReportsResponse {
    pub fn builder() -> ListEcommerceProductActivityReportsResponseBuilder {
        <ListEcommerceProductActivityReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEcommerceProductActivityReportsResponseBuilder {
    links: Option<Vec<ListEcommerceProductActivityReportsResponseLinksItem>>,
    products: Option<Vec<ListEcommerceProductActivityReportsResponseProductsItem>>,
    total_items: Option<i64>,
}

impl ListEcommerceProductActivityReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListEcommerceProductActivityReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<ListEcommerceProductActivityReportsResponseProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEcommerceProductActivityReportsResponse`].
    pub fn build(self) -> Result<ListEcommerceProductActivityReportsResponse, BuildError> {
        Ok(ListEcommerceProductActivityReportsResponse {
            links: self.links,
            products: self.products,
            total_items: self.total_items,
        })
    }
}
