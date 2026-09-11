pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of the interaction with the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OpenActivityOpensItem {
    /// Indicates if the open was from an email client that use proxies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_proxy_open: Option<bool>,
    /// The date and time recorded for the action in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
}

impl OpenActivityOpensItem {
    pub fn builder() -> OpenActivityOpensItemBuilder {
        <OpenActivityOpensItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenActivityOpensItemBuilder {
    is_proxy_open: Option<bool>,
    timestamp: Option<DateTime<FixedOffset>>,
}

impl OpenActivityOpensItemBuilder {
    pub fn is_proxy_open(mut self, value: bool) -> Self {
        self.is_proxy_open = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenActivityOpensItem`].
    pub fn build(self) -> Result<OpenActivityOpensItem, BuildError> {
        Ok(OpenActivityOpensItem {
            is_proxy_open: self.is_proxy_open,
            timestamp: self.timestamp,
        })
    }
}
