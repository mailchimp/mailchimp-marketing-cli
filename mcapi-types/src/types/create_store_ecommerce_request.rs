pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateStoreEcommerceRequest {
    /// The store address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateStoreEcommerceRequestAddress>,
    /// The three-letter ISO 4217 code for the currency that the store accepts.
    #[serde(default)]
    pub currency_code: String,
    /// The store domain. This parameter is required for Connected Sites and Google Ads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The email address for the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The unique identifier for the store.
    #[serde(default)]
    pub id: String,
    /// Whether to disable automations because the store is currently [syncing](https://mailchimp.com/developer/marketing/docs/e-commerce/#pausing-store-automations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_syncing: Option<bool>,
    /// The unique identifier for the list associated with the store. The `list_id` for a specific store cannot change.
    #[serde(default)]
    pub list_id: String,
    /// The currency format for the store. For example: `$`, `£`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_format: Option<String>,
    /// The name of the store.
    #[serde(default)]
    pub name: String,
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

impl CreateStoreEcommerceRequest {
    pub fn builder() -> CreateStoreEcommerceRequestBuilder {
        <CreateStoreEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStoreEcommerceRequestBuilder {
    address: Option<CreateStoreEcommerceRequestAddress>,
    currency_code: Option<String>,
    domain: Option<String>,
    email_address: Option<String>,
    id: Option<String>,
    is_syncing: Option<bool>,
    list_id: Option<String>,
    money_format: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    platform: Option<String>,
    primary_locale: Option<String>,
    timezone: Option<String>,
}

impl CreateStoreEcommerceRequestBuilder {
    pub fn address(mut self, value: CreateStoreEcommerceRequestAddress) -> Self {
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

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_syncing(mut self, value: bool) -> Self {
        self.is_syncing = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateStoreEcommerceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency_code`](CreateStoreEcommerceRequestBuilder::currency_code)
    /// - [`id`](CreateStoreEcommerceRequestBuilder::id)
    /// - [`list_id`](CreateStoreEcommerceRequestBuilder::list_id)
    /// - [`name`](CreateStoreEcommerceRequestBuilder::name)
    pub fn build(self) -> Result<CreateStoreEcommerceRequest, BuildError> {
        Ok(CreateStoreEcommerceRequest {
            address: self.address,
            currency_code: self.currency_code.ok_or_else(|| BuildError::missing_field("currency_code"))?,
            domain: self.domain,
            email_address: self.email_address,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_syncing: self.is_syncing,
            list_id: self.list_id.ok_or_else(|| BuildError::missing_field("list_id"))?,
            money_format: self.money_format,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            phone: self.phone,
            platform: self.platform,
            primary_locale: self.primary_locale,
            timezone: self.timezone,
        })
    }
}

