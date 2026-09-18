pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Details for the automations attached to this store.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommerceStoreAutomations {
    /// abandonedBrowse automation details. abandonedBrowse is also known as Product Retargeting Email or Retarget Site Visitors on the web.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_browse: Option<ECommerceStoreAutomationsAbandonedBrowse>,
    /// abandonedCart automation details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abandoned_cart: Option<ECommerceStoreAutomationsAbandonedCart>,
}

impl ECommerceStoreAutomations {
    pub fn builder() -> ECommerceStoreAutomationsBuilder {
        <ECommerceStoreAutomationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceStoreAutomationsBuilder {
    abandoned_browse: Option<ECommerceStoreAutomationsAbandonedBrowse>,
    abandoned_cart: Option<ECommerceStoreAutomationsAbandonedCart>,
}

impl ECommerceStoreAutomationsBuilder {
    pub fn abandoned_browse(mut self, value: ECommerceStoreAutomationsAbandonedBrowse) -> Self {
        self.abandoned_browse = Some(value);
        self
    }

    pub fn abandoned_cart(mut self, value: ECommerceStoreAutomationsAbandonedCart) -> Self {
        self.abandoned_cart = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceStoreAutomations`].
    pub fn build(self) -> Result<ECommerceStoreAutomations, BuildError> {
        Ok(ECommerceStoreAutomations {
            abandoned_browse: self.abandoned_browse,
            abandoned_cart: self.abandoned_cart,
        })
    }
}
