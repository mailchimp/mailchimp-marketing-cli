pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about a specific connected site.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectedSite {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ConnectedSiteLinksItem>>,
    /// The date and time the connected site was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The connected site domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The unique identifier for the site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_id: Option<String>,
    /// The platform of the connected site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// The script used to connect your site with Mailchimp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_script: Option<ConnectedSiteSiteScript>,
    /// The unique identifier for the ecommerce store that's associated with the connected site (if any). The store_id for a specific connected site can't change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    /// The date and time the connected site was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl ConnectedSite {
    pub fn builder() -> ConnectedSiteBuilder {
        <ConnectedSiteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectedSiteBuilder {
    links: Option<Vec<ConnectedSiteLinksItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    domain: Option<String>,
    foreign_id: Option<String>,
    platform: Option<String>,
    site_script: Option<ConnectedSiteSiteScript>,
    store_id: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ConnectedSiteBuilder {
    pub fn links(mut self, value: Vec<ConnectedSiteLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn foreign_id(mut self, value: impl Into<String>) -> Self {
        self.foreign_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn site_script(mut self, value: ConnectedSiteSiteScript) -> Self {
        self.site_script = Some(value);
        self
    }

    pub fn store_id(mut self, value: impl Into<String>) -> Self {
        self.store_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectedSite`].
    pub fn build(self) -> Result<ConnectedSite, BuildError> {
        Ok(ConnectedSite {
            links: self.links,
            created_at: self.created_at,
            domain: self.domain,
            foreign_id: self.foreign_id,
            platform: self.platform,
            site_script: self.site_script,
            store_id: self.store_id,
            updated_at: self.updated_at,
        })
    }
}
