pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An individual click location.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEepurlReportsResponseClicksLocationsItem {
    /// The two-digit country code for a recorded click.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// If available, a specific region where the click was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

impl ListEepurlReportsResponseClicksLocationsItem {
    pub fn builder() -> ListEepurlReportsResponseClicksLocationsItemBuilder {
        <ListEepurlReportsResponseClicksLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEepurlReportsResponseClicksLocationsItemBuilder {
    country: Option<String>,
    region: Option<String>,
}

impl ListEepurlReportsResponseClicksLocationsItemBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEepurlReportsResponseClicksLocationsItem`].
    pub fn build(self) -> Result<ListEepurlReportsResponseClicksLocationsItem, BuildError> {
        Ok(ListEepurlReportsResponseClicksLocationsItem {
            country: self.country,
            region: self.region,
        })
    }
}
