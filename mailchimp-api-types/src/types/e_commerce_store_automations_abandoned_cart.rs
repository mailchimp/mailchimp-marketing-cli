pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// abandonedCart automation details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommerceStoreAutomationsAbandonedCart {
    /// Unique ID of automation parent campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this store supports the abandonedCart automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supported: Option<bool>,
    /// Status of the abandonedCart automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ECommerceStoreAutomationsAbandonedCartStatus>,
}

impl ECommerceStoreAutomationsAbandonedCart {
    pub fn builder() -> ECommerceStoreAutomationsAbandonedCartBuilder {
        <ECommerceStoreAutomationsAbandonedCartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceStoreAutomationsAbandonedCartBuilder {
    id: Option<String>,
    is_supported: Option<bool>,
    status: Option<ECommerceStoreAutomationsAbandonedCartStatus>,
}

impl ECommerceStoreAutomationsAbandonedCartBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_supported(mut self, value: bool) -> Self {
        self.is_supported = Some(value);
        self
    }

    pub fn status(mut self, value: ECommerceStoreAutomationsAbandonedCartStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceStoreAutomationsAbandonedCart`].
    pub fn build(self) -> Result<ECommerceStoreAutomationsAbandonedCart, BuildError> {
        Ok(ECommerceStoreAutomationsAbandonedCart {
            id: self.id,
            is_supported: self.is_supported,
            status: self.status,
        })
    }
}
