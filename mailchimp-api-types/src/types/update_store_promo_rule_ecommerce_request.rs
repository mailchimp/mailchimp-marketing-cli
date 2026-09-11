pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateStorePromoRuleEcommerceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<UpdateStorePromoRuleEcommerceRequestAmount>,
    /// The date and time the promotion was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_foreign: Option<String>,
    /// The description of a promotion restricted to UTF-8 characters with max length 255.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the promo rule is currently enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<UpdateStorePromoRuleEcommerceRequestEndsAt>,
    /// A unique identifier for the promo rule. If Ecommerce platform does not support promo rule, use promo code id as promo rule id. Restricted to UTF-8 characters with max length 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<UpdateStorePromoRuleEcommerceRequestStartsAt>,
    /// The target that the discount applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<UpdateStorePromoRuleEcommerceRequestTarget>,
    /// The title that will show up in promotion campaign. Restricted to UTF-8 characters with max length of 100 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Type of discount. For free shipping set type to fixed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UpdateStorePromoRuleEcommerceRequestType>,
    /// The date and time the promotion was updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_foreign: Option<String>,
}

impl UpdateStorePromoRuleEcommerceRequest {
    pub fn builder() -> UpdateStorePromoRuleEcommerceRequestBuilder {
        <UpdateStorePromoRuleEcommerceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateStorePromoRuleEcommerceRequestBuilder {
    amount: Option<UpdateStorePromoRuleEcommerceRequestAmount>,
    created_at_foreign: Option<String>,
    description: Option<String>,
    enabled: Option<bool>,
    ends_at: Option<UpdateStorePromoRuleEcommerceRequestEndsAt>,
    id: Option<String>,
    starts_at: Option<UpdateStorePromoRuleEcommerceRequestStartsAt>,
    target: Option<UpdateStorePromoRuleEcommerceRequestTarget>,
    title: Option<String>,
    r#type: Option<UpdateStorePromoRuleEcommerceRequestType>,
    updated_at_foreign: Option<String>,
}

impl UpdateStorePromoRuleEcommerceRequestBuilder {
    pub fn amount(mut self, value: UpdateStorePromoRuleEcommerceRequestAmount) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.created_at_foreign = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn ends_at(mut self, value: UpdateStorePromoRuleEcommerceRequestEndsAt) -> Self {
        self.ends_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn starts_at(mut self, value: UpdateStorePromoRuleEcommerceRequestStartsAt) -> Self {
        self.starts_at = Some(value);
        self
    }

    pub fn target(mut self, value: UpdateStorePromoRuleEcommerceRequestTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: UpdateStorePromoRuleEcommerceRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.updated_at_foreign = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateStorePromoRuleEcommerceRequest`].
    pub fn build(self) -> Result<UpdateStorePromoRuleEcommerceRequest, BuildError> {
        Ok(UpdateStorePromoRuleEcommerceRequest {
            amount: self.amount,
            created_at_foreign: self.created_at_foreign,
            description: self.description,
            enabled: self.enabled,
            ends_at: self.ends_at,
            id: self.id,
            starts_at: self.starts_at,
            target: self.target,
            title: self.title,
            r#type: self.r#type,
            updated_at_foreign: self.updated_at_foreign,
        })
    }
}

