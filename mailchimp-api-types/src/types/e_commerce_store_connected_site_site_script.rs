pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The script used to connect your site with Mailchimp.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommerceStoreConnectedSiteSiteScript {
    /// A pre-built script that you can copy-and-paste into your site to integrate it with Mailchimp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment: Option<String>,
    /// The URL used for any integrations that offer built-in support for connected sites.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ECommerceStoreConnectedSiteSiteScript {
    pub fn builder() -> ECommerceStoreConnectedSiteSiteScriptBuilder {
        <ECommerceStoreConnectedSiteSiteScriptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceStoreConnectedSiteSiteScriptBuilder {
    fragment: Option<String>,
    url: Option<String>,
}

impl ECommerceStoreConnectedSiteSiteScriptBuilder {
    pub fn fragment(mut self, value: impl Into<String>) -> Self {
        self.fragment = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ECommerceStoreConnectedSiteSiteScript`].
    pub fn build(self) -> Result<ECommerceStoreConnectedSiteSiteScript, BuildError> {
        Ok(ECommerceStoreConnectedSiteSiteScript {
            fragment: self.fragment,
            url: self.url,
        })
    }
}
