pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateStorePromoRuleRequestEndsAt {
        UpdateStorePromoRuleRequestEndsAtZero(UpdateStorePromoRuleRequestEndsAtZero),

        UpdateStorePromoRuleRequestEndsAtOne(UpdateStorePromoRuleRequestEndsAtOne),
}

impl UpdateStorePromoRuleRequestEndsAt {
    pub fn is_update_store_promo_rule_request_ends_at_zero(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleRequestEndsAtZero(_))
    }

    pub fn is_update_store_promo_rule_request_ends_at_one(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleRequestEndsAtOne(_))
    }


    pub fn as_update_store_promo_rule_request_ends_at_zero(&self) -> Option<&UpdateStorePromoRuleRequestEndsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_request_ends_at_zero(self) -> Option<UpdateStorePromoRuleRequestEndsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_update_store_promo_rule_request_ends_at_one(&self) -> Option<&UpdateStorePromoRuleRequestEndsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_request_ends_at_one(self) -> Option<UpdateStorePromoRuleRequestEndsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpdateStorePromoRuleRequestEndsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdateStorePromoRuleRequestEndsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UpdateStorePromoRuleRequestEndsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
