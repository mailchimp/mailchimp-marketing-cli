pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateStorePromoRuleEcommerceRequestStartsAt {
        CreateStorePromoRuleEcommerceRequestStartsAtZero(CreateStorePromoRuleEcommerceRequestStartsAtZero),

        CreateStorePromoRuleEcommerceRequestStartsAtOne(CreateStorePromoRuleEcommerceRequestStartsAtOne),
}

impl CreateStorePromoRuleEcommerceRequestStartsAt {
    pub fn is_create_store_promo_rule_ecommerce_request_starts_at_zero(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleEcommerceRequestStartsAtZero(_))
    }

    pub fn is_create_store_promo_rule_ecommerce_request_starts_at_one(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleEcommerceRequestStartsAtOne(_))
    }


    pub fn as_create_store_promo_rule_ecommerce_request_starts_at_zero(&self) -> Option<&CreateStorePromoRuleEcommerceRequestStartsAtZero> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_ecommerce_request_starts_at_zero(self) -> Option<CreateStorePromoRuleEcommerceRequestStartsAtZero> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_create_store_promo_rule_ecommerce_request_starts_at_one(&self) -> Option<&CreateStorePromoRuleEcommerceRequestStartsAtOne> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_ecommerce_request_starts_at_one(self) -> Option<CreateStorePromoRuleEcommerceRequestStartsAtOne> {
        match self {
                    Self::CreateStorePromoRuleEcommerceRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateStorePromoRuleEcommerceRequestStartsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateStorePromoRuleEcommerceRequestStartsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CreateStorePromoRuleEcommerceRequestStartsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
