pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of delay for an Automation email.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationWorkflowEmailDelayType {
    Now,
    Day,
    Hour,
    Week,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationWorkflowEmailDelayType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Now => serializer.serialize_str("now"),
            Self::Day => serializer.serialize_str("day"),
            Self::Hour => serializer.serialize_str("hour"),
            Self::Week => serializer.serialize_str("week"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationWorkflowEmailDelayType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "now" => Ok(Self::Now),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "week" => Ok(Self::Week),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationWorkflowEmailDelayType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Now => write!(f, "now"),
            Self::Day => write!(f, "day"),
            Self::Hour => write!(f, "hour"),
            Self::Week => write!(f, "week"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
