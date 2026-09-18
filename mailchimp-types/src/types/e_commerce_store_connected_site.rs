pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The Connected Site associated with the store.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommerceStoreConnectedSite {
    /// The unique identifier for the connected site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_foreign_id: Option<String>,
    /// The script used to connect your site with Mailchimp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_script: Option<ECommerceStoreConnectedSiteSiteScript>,
}

impl ECommerceStoreConnectedSite {
    pub fn builder() -> ECommerceStoreConnectedSiteBuilder {
        <ECommerceStoreConnectedSiteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceStoreConnectedSiteBuilder {
    site_foreign_id: Option<String>,
    site_script: Option<ECommerceStoreConnectedSiteSiteScript>,
}

impl ECommerceStoreConnectedSiteBuilder {
    pub fn site_foreign_id(mut self, value: impl Into<String>) -> Self {
        self.site_foreign_id = Some(value.into());
        self
    }

    pub fn site_script(mut self, value: ECommerceStoreConnectedSiteSiteScript) -> Self {
        self.site_script = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceStoreConnectedSite`].
    pub fn build(self) -> Result<ECommerceStoreConnectedSite, BuildError> {
        Ok(ECommerceStoreConnectedSite {
            site_foreign_id: self.site_foreign_id,
            site_script: self.site_script,
        })
    }
}
