pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateStorePromoRuleRequestStartsAtOne {
    Empty,
    Zero0000,
    Zero0000000000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateStorePromoRuleRequestStartsAtOne {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Empty => serializer.serialize_str(""),
            Self::Zero0000 => serializer.serialize_str("0000-00-00"),
            Self::Zero0000000000 => serializer.serialize_str("0000-00-00 00:00:00"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateStorePromoRuleRequestStartsAtOne {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "" => Ok(Self::Empty),
            "0000-00-00" => Ok(Self::Zero0000),
            "0000-00-00 00:00:00" => Ok(Self::Zero0000000000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateStorePromoRuleRequestStartsAtOne {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Zero0000 => write!(f, "0000-00-00"),
            Self::Zero0000000000 => write!(f, "0000-00-00 00:00:00"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
