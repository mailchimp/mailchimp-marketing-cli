pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The shipping address for the order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreOrderEcommerceRequestShippingAddress {
    /// The shipping address for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// An additional field for the shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// The city in the order's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The company associated with an order's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// The country in the order's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The two-letter code for the country in the shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// The latitude for the shipping address location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<UpdateStoreOrderEcommerceRequestShippingAddressLatitude>,
    /// The longitude for the shipping address location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<UpdateStoreOrderEcommerceRequestShippingAddressLongitude>,
    /// The name associated with an order's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The phone number for the order's shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The postal or zip code in the order's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The state or normalized province in the order's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// The two-letter code for the province or state the order's shipping address is located in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<String>,
}

impl UpdateStoreOrderEcommerceRequestShippingAddress {
    pub fn builder() -> UpdateStoreOrderEcommerceRequestShippingAddressBuilder {
        <UpdateStoreOrderEcommerceRequestShippingAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreOrderEcommerceRequestShippingAddressBuilder {
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    company: Option<String>,
    country: Option<String>,
    country_code: Option<String>,
    latitude: Option<UpdateStoreOrderEcommerceRequestShippingAddressLatitude>,
    longitude: Option<UpdateStoreOrderEcommerceRequestShippingAddressLongitude>,
    name: Option<String>,
    phone: Option<String>,
    postal_code: Option<String>,
    province: Option<String>,
    province_code: Option<String>,
}

impl UpdateStoreOrderEcommerceRequestShippingAddressBuilder {
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

    pub fn latitude(mut self, value: UpdateStoreOrderEcommerceRequestShippingAddressLatitude) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: UpdateStoreOrderEcommerceRequestShippingAddressLongitude) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateStoreOrderEcommerceRequestShippingAddress`].
    pub fn build(self) -> Result<UpdateStoreOrderEcommerceRequestShippingAddress, BuildError> {
        Ok(UpdateStoreOrderEcommerceRequestShippingAddress {
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
