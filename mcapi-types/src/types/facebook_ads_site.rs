pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Connected Site
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FacebookAdsSite {
    /// The ID of this connected site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// The name of the connected site
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The URL for this connected site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FacebookAdsSite {
    pub fn builder() -> FacebookAdsSiteBuilder {
        <FacebookAdsSiteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsSiteBuilder {
    id: Option<i64>,
    name: Option<String>,
    url: Option<String>,
}

impl FacebookAdsSiteBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsSite`].
    pub fn build(self) -> Result<FacebookAdsSite, BuildError> {
        Ok(FacebookAdsSite {
            id: self.id,
            name: self.name,
            url: self.url,
        })
    }
}
