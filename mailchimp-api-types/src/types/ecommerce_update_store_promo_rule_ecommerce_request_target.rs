pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The target that the discount applies to.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateStorePromoRuleEcommerceRequestTarget {
    PerItem,
    Total,
    Shipping,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateStorePromoRuleEcommerceRequestTarget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PerItem => serializer.serialize_str("per_item"),
            Self::Total => serializer.serialize_str("total"),
            Self::Shipping => serializer.serialize_str("shipping"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateStorePromoRuleEcommerceRequestTarget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "per_item" => Ok(Self::PerItem),
            "total" => Ok(Self::Total),
            "shipping" => Ok(Self::Shipping),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateStorePromoRuleEcommerceRequestTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PerItem => write!(f, "per_item"),
            Self::Total => write!(f, "total"),
            Self::Shipping => write!(f, "shipping"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
