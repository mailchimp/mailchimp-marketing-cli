pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of pricing plan the account is on.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListResponsePricingPlanType {
    Monthly,
    PayAsYouGo,
    ForeverFree,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListResponsePricingPlanType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Monthly => serializer.serialize_str("monthly"),
            Self::PayAsYouGo => serializer.serialize_str("pay_as_you_go"),
            Self::ForeverFree => serializer.serialize_str("forever_free"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListResponsePricingPlanType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "monthly" => Ok(Self::Monthly),
            "pay_as_you_go" => Ok(Self::PayAsYouGo),
            "forever_free" => Ok(Self::ForeverFree),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListResponsePricingPlanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monthly => write!(f, "monthly"),
            Self::PayAsYouGo => write!(f, "pay_as_you_go"),
            Self::ForeverFree => write!(f, "forever_free"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
