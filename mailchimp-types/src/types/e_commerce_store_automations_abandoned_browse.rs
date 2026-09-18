pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// abandonedBrowse automation details. abandonedBrowse is also known as Product Retargeting Email or Retarget Site Visitors on the web.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommerceStoreAutomationsAbandonedBrowse {
    /// Unique ID of automation parent campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether this store supports the abandonedBrowse automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_supported: Option<bool>,
    /// Status of the abandonedBrowse automation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ECommerceStoreAutomationsAbandonedBrowseStatus>,
}

impl ECommerceStoreAutomationsAbandonedBrowse {
    pub fn builder() -> ECommerceStoreAutomationsAbandonedBrowseBuilder {
        <ECommerceStoreAutomationsAbandonedBrowseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceStoreAutomationsAbandonedBrowseBuilder {
    id: Option<String>,
    is_supported: Option<bool>,
    status: Option<ECommerceStoreAutomationsAbandonedBrowseStatus>,
}

impl ECommerceStoreAutomationsAbandonedBrowseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_supported(mut self, value: bool) -> Self {
        self.is_supported = Some(value);
        self
    }

    pub fn status(mut self, value: ECommerceStoreAutomationsAbandonedBrowseStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceStoreAutomationsAbandonedBrowse`].
    pub fn build(self) -> Result<ECommerceStoreAutomationsAbandonedBrowse, BuildError> {
        Ok(ECommerceStoreAutomationsAbandonedBrowse {
            id: self.id,
            is_supported: self.is_supported,
            status: self.status,
        })
    }
}
