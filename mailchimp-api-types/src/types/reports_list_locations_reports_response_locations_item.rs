pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLocationsReportsResponseLocationsItem {
    /// The ISO 3166 2 digit country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The number of unique campaign opens for a region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opens: Option<i64>,
    /// The number of unique campaign opens for a region excluding opens from email clients that use proxies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_excluded_opens: Option<i64>,
    /// An internal code for the region representing the more specific location area such as city or state. When this is blank, it indicates we know the country, but not the region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The name of the region, if we have one. For blank "region" values, this will be "Rest of Country".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

impl ListLocationsReportsResponseLocationsItem {
    pub fn builder() -> ListLocationsReportsResponseLocationsItemBuilder {
        <ListLocationsReportsResponseLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLocationsReportsResponseLocationsItemBuilder {
    country_code: Option<String>,
    opens: Option<i64>,
    proxy_excluded_opens: Option<i64>,
    region: Option<String>,
    region_name: Option<String>,
}

impl ListLocationsReportsResponseLocationsItemBuilder {
    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn opens(mut self, value: i64) -> Self {
        self.opens = Some(value);
        self
    }

    pub fn proxy_excluded_opens(mut self, value: i64) -> Self {
        self.proxy_excluded_opens = Some(value);
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn region_name(mut self, value: impl Into<String>) -> Self {
        self.region_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListLocationsReportsResponseLocationsItem`].
    pub fn build(self) -> Result<ListLocationsReportsResponseLocationsItem, BuildError> {
        Ok(ListLocationsReportsResponseLocationsItem {
            country_code: self.country_code,
            opens: self.opens,
            proxy_excluded_opens: self.proxy_excluded_opens,
            region: self.region,
            region_name: self.region_name,
        })
    }
}
