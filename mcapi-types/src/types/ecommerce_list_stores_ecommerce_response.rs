pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of stores in the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoresEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoresEcommerceResponseLinksItem>>,
    /// An array of objects, each representing a store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stores: Option<Vec<ECommerceStore>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStoresEcommerceResponse {
    pub fn builder() -> ListStoresEcommerceResponseBuilder {
        <ListStoresEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoresEcommerceResponseBuilder {
    links: Option<Vec<ListStoresEcommerceResponseLinksItem>>,
    stores: Option<Vec<ECommerceStore>>,
    total_items: Option<i64>,
}

impl ListStoresEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoresEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn stores(mut self, value: Vec<ECommerceStore>) -> Self {
        self.stores = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStoresEcommerceResponse`].
    pub fn build(self) -> Result<ListStoresEcommerceResponse, BuildError> {
        Ok(ListStoresEcommerceResponse {
            links: self.links,
            stores: self.stores,
            total_items: self.total_items,
        })
    }
}
