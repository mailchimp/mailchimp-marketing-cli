pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateStorePromoRuleRequestStartsAt {
        UpdateStorePromoRuleRequestStartsAtZero(UpdateStorePromoRuleRequestStartsAtZero),

        UpdateStorePromoRuleRequestStartsAtOne(UpdateStorePromoRuleRequestStartsAtOne),
}

impl UpdateStorePromoRuleRequestStartsAt {
    pub fn is_update_store_promo_rule_request_starts_at_zero(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleRequestStartsAtZero(_))
    }

    pub fn is_update_store_promo_rule_request_starts_at_one(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleRequestStartsAtOne(_))
    }


    pub fn as_update_store_promo_rule_request_starts_at_zero(&self) -> Option<&UpdateStorePromoRuleRequestStartsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_request_starts_at_zero(self) -> Option<UpdateStorePromoRuleRequestStartsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_update_store_promo_rule_request_starts_at_one(&self) -> Option<&UpdateStorePromoRuleRequestStartsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_request_starts_at_one(self) -> Option<UpdateStorePromoRuleRequestStartsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpdateStorePromoRuleRequestStartsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdateStorePromoRuleRequestStartsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UpdateStorePromoRuleRequestStartsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
