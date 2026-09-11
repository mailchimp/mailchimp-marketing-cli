pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by amount spent on a single order or across all orders.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEcommSpentField {
    EcommSpentOne,
    EcommSpentAll,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemEcommSpentField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EcommSpentOne => serializer.serialize_str("ecomm_spent_one"),
            Self::EcommSpentAll => serializer.serialize_str("ecomm_spent_all"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemEcommSpentField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ecomm_spent_one" => Ok(Self::EcommSpentOne),
            "ecomm_spent_all" => Ok(Self::EcommSpentAll),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemEcommSpentField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EcommSpentOne => write!(f, "ecomm_spent_one"),
            Self::EcommSpentAll => write!(f, "ecomm_spent_all"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
