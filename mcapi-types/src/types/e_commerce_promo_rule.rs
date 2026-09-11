pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about an Ecommerce Store's specific Promo Rule
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ECommercePromoRule {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ECommercePromoRuleLinksItem>>,
    /// The amount of the promo code discount. If 'type' is 'fixed', the amount is treated as a monetary value. If 'type' is 'percentage', amount must be a decimal value between 0.0 and 1.0, inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// The date and time the promotion was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at_foreign: Option<DateTime<FixedOffset>>,
    /// The description of a promotion restricted to UTF-8 characters with max length 255.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the promo rule is currently enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The date and time when the promotion ends. Must be after starts_at and in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
    /// A unique identifier for the promo rule. If Ecommerce platform does not support promo rule, use promo code id as promo rule id. Restricted to UTF-8 characters with max length 50.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The date and time when the promotion is in effect in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub starts_at: Option<DateTime<FixedOffset>>,
    /// The target that the discount applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<ECommercePromoRuleTarget>,
    /// The title that will show up in promotion campaign. Restricted to UTF-8 characters with max length of 100 bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Type of discount. For free shipping set type to fixed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ECommercePromoRuleType>,
    /// The date and time the promotion was updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at_foreign: Option<DateTime<FixedOffset>>,
}

impl ECommercePromoRule {
    pub fn builder() -> ECommercePromoRuleBuilder {
        <ECommercePromoRuleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ECommercePromoRuleBuilder {
    links: Option<Vec<ECommercePromoRuleLinksItem>>,
    amount: Option<f64>,
    created_at_foreign: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    enabled: Option<bool>,
    ends_at: Option<String>,
    id: Option<String>,
    starts_at: Option<DateTime<FixedOffset>>,
    target: Option<ECommercePromoRuleTarget>,
    title: Option<String>,
    r#type: Option<ECommercePromoRuleType>,
    updated_at_foreign: Option<DateTime<FixedOffset>>,
}

impl ECommercePromoRuleBuilder {
    pub fn links(mut self, value: Vec<ECommercePromoRuleLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn created_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at_foreign = Some(value);
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

    pub fn ends_at(mut self, value: impl Into<String>) -> Self {
        self.ends_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn starts_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.starts_at = Some(value);
        self
    }

    pub fn target(mut self, value: ECommercePromoRuleTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ECommercePromoRuleType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at_foreign(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at_foreign = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ECommercePromoRule`].
    pub fn build(self) -> Result<ECommercePromoRule, BuildError> {
        Ok(ECommercePromoRule {
            links: self.links,
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
