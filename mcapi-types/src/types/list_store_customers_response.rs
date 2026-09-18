pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of the store's customers.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreCustomersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreCustomersResponseLinksItem>>,
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

impl ListStoreCustomersResponse {
    pub fn builder() -> ListStoreCustomersResponseBuilder {
        <ListStoreCustomersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreCustomersResponseBuilder {
    links: Option<Vec<ListStoreCustomersResponseLinksItem>>,
    customers: Option<Vec<ECommerceCustomer>>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStoreCustomersResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreCustomersResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListStoreCustomersResponse`].
    pub fn build(self) -> Result<ListStoreCustomersResponse, BuildError> {
        Ok(ListStoreCustomersResponse {
            links: self.links,
            customers: self.customers,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
