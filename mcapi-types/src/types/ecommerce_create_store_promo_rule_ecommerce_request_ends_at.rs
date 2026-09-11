pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateStorePromoRuleEcommerceRequestEndsAt {
        CreateStorePromoRuleEcommerceRequestEndsAtZero(CreateStorePromoRuleEcommerceRequestEndsAtZero),

        CreateStorePromoRuleEcommerceRequestEndsAtOne(CreateStorePromoRuleEcommerceRequestEndsAtOne),
}

impl CreateStorePromoRuleEcommerceRequestEndsAt {
    pub fn is_create_store_promo_rule_ecommerce_request_ends_at_zero(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleEcommerceRequestEndsAtZero(_))
    }

    pub fn is_create_store_promo_rule_ecommerce_request_ends_at_one(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleEcommerceRequestEndsAtOne(_))
    }


    pub fn as_create_store_promo_rule_ecommerce_request_ends_at_zero(&self) -> Option<&CreateStorePromoRuleEcommerceRequestEndsAtZero> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_ecommerce_request_ends_at_zero(self) -> Option<CreateStorePromoRuleEcommerceRequestEndsAtZero> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_create_store_promo_rule_ecommerce_request_ends_at_one(&self) -> Option<&CreateStorePromoRuleEcommerceRequestEndsAtOne> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_ecommerce_request_ends_at_one(self) -> Option<CreateStorePromoRuleEcommerceRequestEndsAtOne> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateStorePromoRuleEcommerceRequestEndsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateStorePromoRuleEcommerceRequestEndsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CreateStorePromoRuleEcommerceRequestEndsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
