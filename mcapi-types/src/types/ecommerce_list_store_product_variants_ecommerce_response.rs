pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of a product's variants.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStoreProductVariantsEcommerceResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStoreProductVariantsEcommerceResponseLinksItem>>,
    /// The product id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
    /// An array of objects, each representing a product's variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<ECommerceProductVariant>>,
}

impl ListStoreProductVariantsEcommerceResponse {
    pub fn builder() -> ListStoreProductVariantsEcommerceResponseBuilder {
        <ListStoreProductVariantsEcommerceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStoreProductVariantsEcommerceResponseBuilder {
    links: Option<Vec<ListStoreProductVariantsEcommerceResponseLinksItem>>,
    product_id: Option<String>,
    store_id: Option<String>,
    total_items: Option<i64>,
    variants: Option<Vec<ECommerceProductVariant>>,
}

impl ListStoreProductVariantsEcommerceResponseBuilder {
    pub fn links(mut self, value: Vec<ListStoreProductVariantsEcommerceResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
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

    pub fn variants(mut self, value: Vec<ECommerceProductVariant>) -> Self {
        self.variants = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListStoreProductVariantsEcommerceResponse`].
    pub fn build(self) -> Result<ListStoreProductVariantsEcommerceResponse, BuildError> {
        Ok(ListStoreProductVariantsEcommerceResponse {
            links: self.links,
            product_id: self.product_id,
            store_id: self.store_id,
            total_items: self.total_items,
            variants: self.variants,
        })
    }
}
