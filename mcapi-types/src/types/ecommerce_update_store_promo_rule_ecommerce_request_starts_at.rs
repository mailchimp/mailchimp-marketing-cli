pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateStorePromoRuleEcommerceRequestStartsAt {
        UpdateStorePromoRuleEcommerceRequestStartsAtZero(UpdateStorePromoRuleEcommerceRequestStartsAtZero),

        UpdateStorePromoRuleEcommerceRequestStartsAtOne(UpdateStorePromoRuleEcommerceRequestStartsAtOne),
}

impl UpdateStorePromoRuleEcommerceRequestStartsAt {
    pub fn is_update_store_promo_rule_ecommerce_request_starts_at_zero(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleEcommerceRequestStartsAtZero(_))
    }

    pub fn is_update_store_promo_rule_ecommerce_request_starts_at_one(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleEcommerceRequestStartsAtOne(_))
    }


    pub fn as_update_store_promo_rule_ecommerce_request_starts_at_zero(&self) -> Option<&UpdateStorePromoRuleEcommerceRequestStartsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_ecommerce_request_starts_at_zero(self) -> Option<UpdateStorePromoRuleEcommerceRequestStartsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestStartsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_update_store_promo_rule_ecommerce_request_starts_at_one(&self) -> Option<&UpdateStorePromoRuleEcommerceRequestStartsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_ecommerce_request_starts_at_one(self) -> Option<UpdateStorePromoRuleEcommerceRequestStartsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestStartsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpdateStorePromoRuleEcommerceRequestStartsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdateStorePromoRuleEcommerceRequestStartsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UpdateStorePromoRuleEcommerceRequestStartsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
