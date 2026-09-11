pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateStorePromoRuleEcommerceRequestEndsAt {
        UpdateStorePromoRuleEcommerceRequestEndsAtZero(UpdateStorePromoRuleEcommerceRequestEndsAtZero),

        UpdateStorePromoRuleEcommerceRequestEndsAtOne(UpdateStorePromoRuleEcommerceRequestEndsAtOne),
}

impl UpdateStorePromoRuleEcommerceRequestEndsAt {
    pub fn is_update_store_promo_rule_ecommerce_request_ends_at_zero(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleEcommerceRequestEndsAtZero(_))
    }

    pub fn is_update_store_promo_rule_ecommerce_request_ends_at_one(&self) -> bool {
        matches!(self, Self::UpdateStorePromoRuleEcommerceRequestEndsAtOne(_))
    }


    pub fn as_update_store_promo_rule_ecommerce_request_ends_at_zero(&self) -> Option<&UpdateStorePromoRuleEcommerceRequestEndsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_ecommerce_request_ends_at_zero(self) -> Option<UpdateStorePromoRuleEcommerceRequestEndsAtZero> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestEndsAtZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_update_store_promo_rule_ecommerce_request_ends_at_one(&self) -> Option<&UpdateStorePromoRuleEcommerceRequestEndsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_store_promo_rule_ecommerce_request_ends_at_one(self) -> Option<UpdateStorePromoRuleEcommerceRequestEndsAtOne> {
        match self {
                    Self::UpdateStorePromoRuleEcommerceRequestEndsAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpdateStorePromoRuleEcommerceRequestEndsAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UpdateStorePromoRuleEcommerceRequestEndsAtZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::UpdateStorePromoRuleEcommerceRequestEndsAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
