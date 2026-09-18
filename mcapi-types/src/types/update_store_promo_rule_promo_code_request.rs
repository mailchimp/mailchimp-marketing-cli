pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateStorePromoRulePromoCodeRequest {
    /// The discount code. Restricted to UTF-8 characters with max length 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// The date and time the promotion was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_foreign: Option<String>,
    /// Whether the promo code is currently enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A unique identifier for the promo code. Restricted to UTF-8 characters with max length 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The url that should be used in the promotion campaign restricted to UTF-8 characters with max length 2000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redemption_url: Option<String>,
    /// The date and time the promotion was updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_foreign: Option<String>,
    /// Number of times promo code has been used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i64>,
}

impl UpdateStorePromoRulePromoCodeRequest {
    pub fn builder() -> UpdateStorePromoRulePromoCodeRequestBuilder {
        <UpdateStorePromoRulePromoCodeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStorePromoRulePromoCodeRequestBuilder {
    code: Option<String>,
    created_at_foreign: Option<String>,
    enabled: Option<bool>,
    id: Option<String>,
    redemption_url: Option<String>,
    updated_at_foreign: Option<String>,
    usage_count: Option<i64>,
}

impl UpdateStorePromoRulePromoCodeRequestBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn created_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.created_at_foreign = Some(value.into());
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

    pub fn updated_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.updated_at_foreign = Some(value.into());
        self
    }

    pub fn usage_count(mut self, value: i64) -> Self {
        self.usage_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateStorePromoRulePromoCodeRequest`].
    pub fn build(self) -> Result<UpdateStorePromoRulePromoCodeRequest, BuildError> {
        Ok(UpdateStorePromoRulePromoCodeRequest {
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

