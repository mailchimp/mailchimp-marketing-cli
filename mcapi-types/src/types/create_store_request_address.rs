pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The store address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateStoreRequestAddress {
    /// The store's mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// An additional field for the store's mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// The city the store is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The store's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The two-letter code for to the store's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The latitude of the store location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<CreateStoreRequestAddressLatitude>,
    /// The longitude of the store location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<CreateStoreRequestAddressLongitude>,
    /// The store's postal or zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The store's state name or normalized province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// The two-letter code for the store's province or state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
}

impl CreateStoreRequestAddress {
    pub fn builder() -> CreateStoreRequestAddressBuilder {
        <CreateStoreRequestAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreRequestAddressBuilder {
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    country: Option<String>,
    country_code: Option<String>,
    latitude: Option<CreateStoreRequestAddressLatitude>,
    longitude: Option<CreateStoreRequestAddressLongitude>,
    postal_code: Option<String>,
    province: Option<String>,
    province_code: Option<String>,
}

impl CreateStoreRequestAddressBuilder {
    pub fn address1(mut self, value: impl Into<String>) -> Self {
        self.address1 = Some(value.into());
        self
    }

    pub fn address2(mut self, value: impl Into<String>) -> Self {
        self.address2 = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn country_code(mut self, value: impl Into<String>) -> Self {
        self.country_code = Some(value.into());
        self
    }

    pub fn latitude(mut self, value: CreateStoreRequestAddressLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: CreateStoreRequestAddressLongitude) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn province(mut self, value: impl Into<String>) -> Self {
        self.province = Some(value.into());
        self
    }

    pub fn province_code(mut self, value: impl Into<String>) -> Self {
        self.province_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateStoreRequestAddress`].
    pub fn build(self) -> Result<CreateStoreRequestAddress, BuildError> {
        Ok(CreateStoreRequestAddress {
            address1: self.address1,
            address2: self.address2,
            city: self.city,
            country: self.country,
            country_code: self.country_code,
            latitude: self.latitude,
            longitude: self.longitude,
            postal_code: self.postal_code,
            province: self.province,
            province_code: self.province_code,
        })
    }
}
