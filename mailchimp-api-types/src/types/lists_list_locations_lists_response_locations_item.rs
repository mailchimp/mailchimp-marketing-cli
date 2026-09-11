pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListLocationsListsResponseLocationsItem {
    /// The ISO 3166 2 digit country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    /// The name of the country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The percent of subscribers in the country.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percent: Option<f64>,
    /// The total number of subscribers in the country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

impl ListLocationsListsResponseLocationsItem {
    pub fn builder() -> ListLocationsListsResponseLocationsItemBuilder {
        <ListLocationsListsResponseLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLocationsListsResponseLocationsItemBuilder {
    cc: Option<String>,
    country: Option<String>,
    percent: Option<f64>,
    total: Option<i64>,
}

impl ListLocationsListsResponseLocationsItemBuilder {
    pub fn cc(mut self, value: impl Into<String>) -> Self {
        self.cc = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn percent(mut self, value: f64) -> Self {
        self.percent = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLocationsListsResponseLocationsItem`].
    pub fn build(self) -> Result<ListLocationsListsResponseLocationsItem, BuildError> {
        Ok(ListLocationsListsResponseLocationsItem {
            cc: self.cc,
            country: self.country,
            percent: self.percent,
            total: self.total,
        })
    }
}
