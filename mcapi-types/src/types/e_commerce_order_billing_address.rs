pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The billing address for the order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceOrderBillingAddress {
    /// The billing address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// An additional field for the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// The city in the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The company associated with the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The country in the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The two-letter code for the country in the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The latitude for the billing address location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub latitude: Option<f64>,
    /// The longitude for the billing address location.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub longitude: Option<f64>,
    /// The name associated with an order's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The phone number for the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The postal or zip code in the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state or normalized province in the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// The two-letter code for the province or state in the billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
}

impl ECommerceOrderBillingAddress {
    pub fn builder() -> ECommerceOrderBillingAddressBuilder {
        <ECommerceOrderBillingAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceOrderBillingAddressBuilder {
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    company: Option<String>,
    country: Option<String>,
    country_code: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    name: Option<String>,
    phone: Option<String>,
    postal_code: Option<String>,
    province: Option<String>,
    province_code: Option<String>,
}

impl ECommerceOrderBillingAddressBuilder {
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

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
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

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
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

    /// Consumes the builder and constructs a [`ECommerceOrderBillingAddress`].
    pub fn build(self) -> Result<ECommerceOrderBillingAddress, BuildError> {
        Ok(ECommerceOrderBillingAddress {
            address1: self.address1,
            address2: self.address2,
            city: self.city,
            company: self.company,
            country: self.country,
            country_code: self.country_code,
            latitude: self.latitude,
            longitude: self.longitude,
            name: self.name,
            phone: self.phone,
            postal_code: self.postal_code,
            province: self.province,
            province_code: self.province_code,
        })
    }
}
