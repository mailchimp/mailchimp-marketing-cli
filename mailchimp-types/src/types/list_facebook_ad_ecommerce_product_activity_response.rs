pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of ecommerce products.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFacebookAdEcommerceProductActivityResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFacebookAdEcommerceProductActivityResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<ListFacebookAdEcommerceProductActivityResponseProductsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFacebookAdEcommerceProductActivityResponse {
    pub fn builder() -> ListFacebookAdEcommerceProductActivityResponseBuilder {
        <ListFacebookAdEcommerceProductActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFacebookAdEcommerceProductActivityResponseBuilder {
    links: Option<Vec<ListFacebookAdEcommerceProductActivityResponseLinksItem>>,
    products: Option<Vec<ListFacebookAdEcommerceProductActivityResponseProductsItem>>,
    total_items: Option<i64>,
}

impl ListFacebookAdEcommerceProductActivityResponseBuilder {
    pub fn links(mut self, value: Vec<ListFacebookAdEcommerceProductActivityResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<ListFacebookAdEcommerceProductActivityResponseProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFacebookAdEcommerceProductActivityResponse`].
    pub fn build(self) -> Result<ListFacebookAdEcommerceProductActivityResponse, BuildError> {
        Ok(ListFacebookAdEcommerceProductActivityResponse {
            links: self.links,
            products: self.products,
            total_items: self.total_items,
        })
    }
}
