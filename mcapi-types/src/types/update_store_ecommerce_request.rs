pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStoreEcommerceRequest {
    /// The store address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateStoreEcommerceRequestAddress>,
    /// The three-letter ISO 4217 code for the currency that the store accepts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The store domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The email address for the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// Whether to disable automations because the store is currently [syncing](https://mailchimp.com/developer/marketing/docs/e-commerce/#pausing-store-automations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_syncing: Option<bool>,
    /// The currency format for the store. For example: `$`, `£`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_format: Option<String>,
    /// The name of the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The store phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The e-commerce platform of the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// The primary locale for the store. For example: `en`, `de`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_locale: Option<String>,
    /// The timezone for the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl UpdateStoreEcommerceRequest {
    pub fn builder() -> UpdateStoreEcommerceRequestBuilder {
        <UpdateStoreEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStoreEcommerceRequestBuilder {
    address: Option<UpdateStoreEcommerceRequestAddress>,
    currency_code: Option<String>,
    domain: Option<String>,
    email_address: Option<String>,
    is_syncing: Option<bool>,
    money_format: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    platform: Option<String>,
    primary_locale: Option<String>,
    timezone: Option<String>,
}

impl UpdateStoreEcommerceRequestBuilder {
    pub fn address(mut self, value: UpdateStoreEcommerceRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn is_syncing(mut self, value: bool) -> Self {
        self.is_syncing = Some(value);
        self
    }

    pub fn money_format(mut self, value: impl Into<String>) -> Self {
        self.money_format = Some(value.into());
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

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn primary_locale(mut self, value: impl Into<String>) -> Self {
        self.primary_locale = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateStoreEcommerceRequest`].
    pub fn build(self) -> Result<UpdateStoreEcommerceRequest, BuildError> {
        Ok(UpdateStoreEcommerceRequest {
            address: self.address,
            currency_code: self.currency_code,
            domain: self.domain,
            email_address: self.email_address,
            is_syncing: self.is_syncing,
            money_format: self.money_format,
            name: self.name,
            phone: self.phone,
            platform: self.platform,
            primary_locale: self.primary_locale,
            timezone: self.timezone,
        })
    }
}

