pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The customer's address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertStoreCustomerRequestAddress {
    /// The mailing address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// An additional field for the customer's mailing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// The city the customer is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The customer's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The two-letter code for the customer's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The customer's postal or zip code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The customer's state name or normalized province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// The two-letter code for the customer's province or state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
}

impl UpsertStoreCustomerRequestAddress {
    pub fn builder() -> UpsertStoreCustomerRequestAddressBuilder {
        <UpsertStoreCustomerRequestAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertStoreCustomerRequestAddressBuilder {
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    country: Option<String>,
    country_code: Option<String>,
    postal_code: Option<String>,
    province: Option<String>,
    province_code: Option<String>,
}

impl UpsertStoreCustomerRequestAddressBuilder {
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

    /// Consumes the builder and constructs a [`UpsertStoreCustomerRequestAddress`].
    pub fn build(self) -> Result<UpsertStoreCustomerRequestAddress, BuildError> {
        Ok(UpsertStoreCustomerRequestAddress {
            address1: self.address1,
            address2: self.address2,
            city: self.city,
            country: self.country,
            country_code: self.country_code,
            postal_code: self.postal_code,
            province: self.province,
            province_code: self.province_code,
        })
    }
}
