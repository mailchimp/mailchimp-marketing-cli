pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of ecommerce products.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListFacebookAdEcommerceProductActivityReportingResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFacebookAdEcommerceProductActivityReportingResponseLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<ListFacebookAdEcommerceProductActivityReportingResponseProductsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFacebookAdEcommerceProductActivityReportingResponse {
    pub fn builder() -> ListFacebookAdEcommerceProductActivityReportingResponseBuilder {
        <ListFacebookAdEcommerceProductActivityReportingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFacebookAdEcommerceProductActivityReportingResponseBuilder {
    links: Option<Vec<ListFacebookAdEcommerceProductActivityReportingResponseLinksItem>>,
    products: Option<Vec<ListFacebookAdEcommerceProductActivityReportingResponseProductsItem>>,
    total_items: Option<i64>,
}

impl ListFacebookAdEcommerceProductActivityReportingResponseBuilder {
    pub fn links(mut self, value: Vec<ListFacebookAdEcommerceProductActivityReportingResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn products(mut self, value: Vec<ListFacebookAdEcommerceProductActivityReportingResponseProductsItem>) -> Self {
        self.products = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFacebookAdEcommerceProductActivityReportingResponse`].
    pub fn build(self) -> Result<ListFacebookAdEcommerceProductActivityReportingResponse, BuildError> {
        Ok(ListFacebookAdEcommerceProductActivityReportingResponse {
            links: self.links,
            products: self.products,
            total_items: self.total_items,
        })
    }
}
