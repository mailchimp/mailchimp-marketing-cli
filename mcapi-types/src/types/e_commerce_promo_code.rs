pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about an Ecommerce Store's specific Promo Code
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ECommercePromoCode {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommercePromoCodeLinksItem>>,
    /// The discount code. Restricted to UTF-8 characters with max length 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The date and time the promotion was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at_foreign: Option<DateTime<FixedOffset>>,
    /// Whether the promo code is currently enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A unique identifier for the promo Code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The url that should be used in the promotion campaign restricted to UTF-8 characters with max length 2000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redemption_url: Option<String>,
    /// The date and time the promotion was updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at_foreign: Option<DateTime<FixedOffset>>,
    /// Number of times promo code has been used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i64>,
}

impl ECommercePromoCode {
    pub fn builder() -> ECommercePromoCodeBuilder {
        <ECommercePromoCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommercePromoCodeBuilder {
    links: Option<Vec<ECommercePromoCodeLinksItem>>,
    code: Option<String>,
    created_at_foreign: Option<DateTime<FixedOffset>>,
    enabled: Option<bool>,
    id: Option<String>,
    redemption_url: Option<String>,
    updated_at_foreign: Option<DateTime<FixedOffset>>,
    usage_count: Option<i64>,
}

impl ECommercePromoCodeBuilder {
    pub fn links(mut self, value: Vec<ECommercePromoCodeLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn created_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at_foreign = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn redemption_url(mut self, value: impl Into<String>) -> Self {
        self.redemption_url = Some(value.into());
        self
    }

    pub fn updated_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at_foreign = Some(value);
        self
    }

    pub fn usage_count(mut self, value: i64) -> Self {
        self.usage_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommercePromoCode`].
    pub fn build(self) -> Result<ECommercePromoCode, BuildError> {
        Ok(ECommercePromoCode {
            links: self.links,
            code: self.code,
            created_at_foreign: self.created_at_foreign,
            enabled: self.enabled,
            id: self.id,
            redemption_url: self.redemption_url,
            updated_at_foreign: self.updated_at_foreign,
            usage_count: self.usage_count,
        })
    }
}
