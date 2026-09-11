pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of the store's customers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreCustomersEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreCustomersEcommerceResponseLinksItem>>,
    /// An array of objects, each representing a customer of a store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customers: Option<Vec<ECommerceCustomer>>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStoreCustomersEcommerceResponse {
    pub fn builder() -> ListStoreCustomersEcommerceResponseBuilder {
        <ListStoreCustomersEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreCustomersEcommerceResponseBuilder {
    links: Option<Vec<ListStoreCustomersEcommerceResponseLinksItem>>,
    customers: Option<Vec<ECommerceCustomer>>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreCustomersEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreCustomersEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn customers(mut self, value: Vec<ECommerceCustomer>) -> Self {
        self.customers = Some(value);
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

    /// Consumes the builder and constructs a [`ListStoreCustomersEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreCustomersEcommerceResponse, BuildError> {
        Ok(ListStoreCustomersEcommerceResponse {
            links: self.links,
            customers: self.customers,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
