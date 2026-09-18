pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of the store's promo codes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListStorePromoRulePromoCodesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStorePromoRulePromoCodesResponseLinksItem>>,
    /// An array of objects, each representing promo codes defined for a store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_codes: Option<Vec<ECommercePromoCode>>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStorePromoRulePromoCodesResponse {
    pub fn builder() -> ListStorePromoRulePromoCodesResponseBuilder {
        <ListStorePromoRulePromoCodesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStorePromoRulePromoCodesResponseBuilder {
    links: Option<Vec<ListStorePromoRulePromoCodesResponseLinksItem>>,
    promo_codes: Option<Vec<ECommercePromoCode>>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStorePromoRulePromoCodesResponseBuilder {
    pub fn links(mut self, value: Vec<ListStorePromoRulePromoCodesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn promo_codes(mut self, value: Vec<ECommercePromoCode>) -> Self {
        self.promo_codes = Some(value);
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

    /// Consumes the builder and constructs a [`ListStorePromoRulePromoCodesResponse`].
    pub fn build(self) -> Result<ListStorePromoRulePromoCodesResponse, BuildError> {
        Ok(ListStorePromoRulePromoCodesResponse {
            links: self.links,
            promo_codes: self.promo_codes,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
