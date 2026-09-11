pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An individual store in an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommerceStore {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommerceStoreLinksItem>>,
    /// The store address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<ECommerceStoreAddress>,
    /// Details for the automations attached to this store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automations: Option<ECommerceStoreAutomations>,
    /// The Connected Site associated with the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_site: Option<ECommerceStoreConnectedSite>,
    /// The date and time the store was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The three-letter ISO 4217 code for the currency that the store accepts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// The store domain.  The store domain must be unique within a user account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    /// The email address for the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    /// The unique identifier for the store.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Whether to disable automations because the store is currently [syncing](https://mailchimp.com/developer/marketing/docs/e-commerce/#pausing-store-automations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_syncing: Option<bool>,
    /// The unique identifier for the list that's associated with the store. The `list_id` for a specific store can't change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list connected to the store, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
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
    /// The date and time the store was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl ECommerceStore {
    pub fn builder() -> ECommerceStoreBuilder {
        <ECommerceStoreBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommerceStoreBuilder {
    links: Option<Vec<ECommerceStoreLinksItem>>,
    address: Option<ECommerceStoreAddress>,
    automations: Option<ECommerceStoreAutomations>,
    connected_site: Option<ECommerceStoreConnectedSite>,
    created_at: Option<DateTime<FixedOffset>>,
    currency_code: Option<String>,
    domain: Option<String>,
    email_address: Option<String>,
    id: Option<String>,
    is_syncing: Option<bool>,
    list_id: Option<String>,
    list_is_active: Option<bool>,
    money_format: Option<String>,
    name: Option<String>,
    phone: Option<String>,
    platform: Option<String>,
    primary_locale: Option<String>,
    timezone: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ECommerceStoreBuilder {
    pub fn links(mut self, value: Vec<ECommerceStoreLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn address(mut self, value: ECommerceStoreAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn automations(mut self, value: ECommerceStoreAutomations) -> Self {
        self.automations = Some(value);
        self
    }

    pub fn connected_site(mut self, value: ECommerceStoreConnectedSite) -> Self {
        self.connected_site = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn list_is_active(mut self, value: bool) -> Self {
        self.list_is_active = Some(value);
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

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommerceStore`].
    pub fn build(self) -> Result<ECommerceStore, BuildError> {
        Ok(ECommerceStore {
            links: self.links,
            address: self.address,
            automations: self.automations,
            connected_site: self.connected_site,
            created_at: self.created_at,
            currency_code: self.currency_code,
            domain: self.domain,
            email_address: self.email_address,
            id: self.id,
            is_syncing: self.is_syncing,
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            money_format: self.money_format,
            name: self.name,
            phone: self.phone,
            platform: self.platform,
            primary_locale: self.primary_locale,
            timezone: self.timezone,
            updated_at: self.updated_at,
        })
    }
}
