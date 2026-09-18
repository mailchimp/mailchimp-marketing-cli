pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateStorePromoRuleRequest {
    pub amount: CreateStorePromoRuleRequestAmount,
    /// The date and time the promotion was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at_foreign: Option<String>,
    /// The description of a promotion restricted to UTF-8 characters with max length 255.
    #[serde(default)]
    pub description: String,
    /// Whether the promo rule is currently enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<CreateStorePromoRuleRequestEndsAt>,
    /// A unique identifier for the promo rule. If Ecommerce platform does not support promo rule, use promo code id as promo rule id. Restricted to UTF-8 characters with max length 50.
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<CreateStorePromoRuleRequestStartsAt>,
    /// The target that the discount applies to.
    pub target: CreateStorePromoRuleRequestTarget,
    /// The title that will show up in promotion campaign. Restricted to UTF-8 characters with max length of 100 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Type of discount. For free shipping set type to fixed.
    pub r#type: CreateStorePromoRuleRequestType,
    /// The date and time the promotion was updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_foreign: Option<String>,
}

impl CreateStorePromoRuleRequest {
    pub fn builder() -> CreateStorePromoRuleRequestBuilder {
        <CreateStorePromoRuleRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateStorePromoRuleRequestBuilder {
    amount: Option<CreateStorePromoRuleRequestAmount>,
    created_at_foreign: Option<String>,
    description: Option<String>,
    enabled: Option<bool>,
    ends_at: Option<CreateStorePromoRuleRequestEndsAt>,
    id: Option<String>,
    starts_at: Option<CreateStorePromoRuleRequestStartsAt>,
    target: Option<CreateStorePromoRuleRequestTarget>,
    title: Option<String>,
    r#type: Option<CreateStorePromoRuleRequestType>,
    updated_at_foreign: Option<String>,
}

impl CreateStorePromoRuleRequestBuilder {
    pub fn amount(mut self, value: CreateStorePromoRuleRequestAmount) -> Self {
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

    pub fn ends_at(mut self, value: CreateStorePromoRuleRequestEndsAt) -> Self {
        self.ends_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn starts_at(mut self, value: CreateStorePromoRuleRequestStartsAt) -> Self {
        self.starts_at = Some(value);
        self
    }

    pub fn target(mut self, value: CreateStorePromoRuleRequestTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateStorePromoRuleRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at_foreign(mut self, value: impl Into<String>) -> Self {
        self.updated_at_foreign = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateStorePromoRuleRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateStorePromoRuleRequestBuilder::amount)
    /// - [`description`](CreateStorePromoRuleRequestBuilder::description)
    /// - [`id`](CreateStorePromoRuleRequestBuilder::id)
    /// - [`target`](CreateStorePromoRuleRequestBuilder::target)
    /// - [`r#type`](CreateStorePromoRuleRequestBuilder::r#type)
    pub fn build(self) -> Result<CreateStorePromoRuleRequest, BuildError> {
        Ok(CreateStorePromoRuleRequest {
            amount: self.amount.ok_or_else(|| BuildError::missing_field("amount"))?,
            created_at_foreign: self.created_at_foreign,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            enabled: self.enabled,
            ends_at: self.ends_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            starts_at: self.starts_at,
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
            title: self.title,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            updated_at_foreign: self.updated_at_foreign,
        })
    }
}

