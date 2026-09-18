pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateStorePromoRuleRequestStartsAt {
        CreateStorePromoRuleRequestStartsAtZero(CreateStorePromoRuleRequestStartsAtZero),

        CreateStorePromoRuleRequestStartsAtOne(CreateStorePromoRuleRequestStartsAtOne),
}

impl CreateStorePromoRuleRequestStartsAt {
    pub fn is_create_store_promo_rule_request_starts_at_zero(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleRequestStartsAtZero(_))
    }

    pub fn is_create_store_promo_rule_request_starts_at_one(&self) -> bool {
        matches!(self, Self::CreateStorePromoRuleRequestStartsAtOne(_))
    }


    pub fn as_create_store_promo_rule_request_starts_at_zero(&self) -> Option<&CreateStorePromoRuleRequestStartsAtZero> {
        match self {
                    Self::CreateStorePromoRuleRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_request_starts_at_zero(self) -> Option<CreateStorePromoRuleRequestStartsAtZero> {
        match self {
                    Self::CreateStorePromoRuleRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_create_store_promo_rule_request_starts_at_one(&self) -> Option<&CreateStorePromoRuleRequestStartsAtOne> {
        match self {
                    Self::CreateStorePromoRuleRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_store_promo_rule_request_starts_at_one(self) -> Option<CreateStorePromoRuleRequestStartsAtOne> {
        match self {
                    Self::CreateStorePromoRuleRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateStorePromoRuleRequestStartsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateStorePromoRuleRequestStartsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CreateStorePromoRuleRequestStartsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
