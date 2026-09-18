pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateStorePromoRuleRequestEndsAt {
        CreateStorePromoRuleRequestEndsAtZero(CreateStorePromoRuleRequestEndsAtZero),

        CreateStorePromoRuleRequestEndsAtOne(CreateStorePromoRuleRequestEndsAtOne),
}

impl CreateStorePromoRuleRequestEndsAt {
    pub fn is_create_store_promo_rule_request_ends_at_zero(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleRequestEndsAtZero(_))
    }

    pub fn is_create_store_promo_rule_request_ends_at_one(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleRequestEndsAtOne(_))
    }


    pub fn as_create_store_promo_rule_request_ends_at_zero(&self) -> Option<&CreateStorePromoRuleRequestEndsAtZero> {
        match self {
                    Self::CreateStorePromoRuleRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_request_ends_at_zero(self) -> Option<CreateStorePromoRuleRequestEndsAtZero> {
        match self {
                    Self::CreateStorePromoRuleRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_create_store_promo_rule_request_ends_at_one(&self) -> Option<&CreateStorePromoRuleRequestEndsAtOne> {
        match self {
                    Self::CreateStorePromoRuleRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_request_ends_at_one(self) -> Option<CreateStorePromoRuleRequestEndsAtOne> {
        match self {
                    Self::CreateStorePromoRuleRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateStorePromoRuleRequestEndsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateStorePromoRuleRequestEndsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CreateStorePromoRuleRequestEndsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
