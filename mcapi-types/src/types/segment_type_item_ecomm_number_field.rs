pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by average spent total, number of orders, total number of products purchased, or average number of products per order.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEcommNumberField {
    EcommSpentAvg,
    EcommOrders,
    EcommProdAll,
    EcommAvgOrd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemEcommNumberField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EcommSpentAvg => serializer.serialize_str("ecomm_spent_avg"),
            Self::EcommOrders => serializer.serialize_str("ecomm_orders"),
            Self::EcommProdAll => serializer.serialize_str("ecomm_prod_all"),
            Self::EcommAvgOrd => serializer.serialize_str("ecomm_avg_ord"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemEcommNumberField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ecomm_spent_avg" => Ok(Self::EcommSpentAvg),
            "ecomm_orders" => Ok(Self::EcommOrders),
            "ecomm_prod_all" => Ok(Self::EcommProdAll),
            "ecomm_avg_ord" => Ok(Self::EcommAvgOrd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemEcommNumberField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EcommSpentAvg => write!(f, "ecomm_spent_avg"),
            Self::EcommOrders => write!(f, "ecomm_orders"),
            Self::EcommProdAll => write!(f, "ecomm_prod_all"),
            Self::EcommAvgOrd => write!(f, "ecomm_avg_ord"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
