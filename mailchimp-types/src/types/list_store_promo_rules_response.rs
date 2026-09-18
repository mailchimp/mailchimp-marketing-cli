pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of the store's promo rules.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListStorePromoRulesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListStorePromoRulesResponseLinksItem>>,
    /// An array of objects, each representing promo rules defined for a store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_rules: Option<Vec<ECommercePromoRule>>,
    /// The store id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListStorePromoRulesResponse {
    pub fn builder() -> ListStorePromoRulesResponseBuilder {
        <ListStorePromoRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListStorePromoRulesResponseBuilder {
    links: Option<Vec<ListStorePromoRulesResponseLinksItem>>,
    promo_rules: Option<Vec<ECommercePromoRule>>,
    store_id: Option<String>,
    total_items: Option<i64>,
}

impl ListStorePromoRulesResponseBuilder {
    pub fn links(mut self, value: Vec<ListStorePromoRulesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn promo_rules(mut self, value: Vec<ECommercePromoRule>) -> Self {
        self.promo_rules = Some(value);
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

    /// Consumes the builder and constructs a [`ListStorePromoRulesResponse`].
    pub fn build(self) -> Result<ListStorePromoRulesResponse, BuildError> {
        Ok(ListStorePromoRulesResponse {
            links: self.links,
            promo_rules: self.promo_rules,
            store_id: self.store_id,
            total_items: self.total_items,
        })
    }
}
