pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FacebookAdsAudienceTargetingSpecsLocations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cities: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zips: Option<Vec<String>>,
}

impl FacebookAdsAudienceTargetingSpecsLocations {
    pub fn builder() -> FacebookAdsAudienceTargetingSpecsLocationsBuilder {
        <FacebookAdsAudienceTargetingSpecsLocationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsAudienceTargetingSpecsLocationsBuilder {
    cities: Option<Vec<String>>,
    countries: Option<Vec<String>>,
    regions: Option<Vec<String>>,
    zips: Option<Vec<String>>,
}

impl FacebookAdsAudienceTargetingSpecsLocationsBuilder {
    pub fn cities(mut self, value: Vec<String>) -> Self {
        self.cities = Some(value);
        self
    }

    pub fn countries(mut self, value: Vec<String>) -> Self {
        self.countries = Some(value);
        self
    }

    pub fn regions(mut self, value: Vec<String>) -> Self {
        self.regions = Some(value);
        self
    }

    pub fn zips(mut self, value: Vec<String>) -> Self {
        self.zips = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsAudienceTargetingSpecsLocations`].
    pub fn build(self) -> Result<FacebookAdsAudienceTargetingSpecsLocations, BuildError> {
        Ok(FacebookAdsAudienceTargetingSpecsLocations {
            cities: self.cities,
            countries: self.countries,
            regions: self.regions,
            zips: self.zips,
        })
    }
}
