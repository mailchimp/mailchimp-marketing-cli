pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the delay settings describe before or after the delay action of an Automation email.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationWorkflowEmailDelayDirection {
    Before,
    After,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationWorkflowEmailDelayDirection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Before => serializer.serialize_str("before"),
            Self::After => serializer.serialize_str("after"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationWorkflowEmailDelayDirection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "before" => Ok(Self::Before),
            "after" => Ok(Self::After),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationWorkflowEmailDelayDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Before => write!(f, "before"),
            Self::After => write!(f, "after"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
