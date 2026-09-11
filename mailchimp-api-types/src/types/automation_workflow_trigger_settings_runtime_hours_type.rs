pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// When to send the Automation email.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationWorkflowTriggerSettingsRuntimeHoursType {
    SendAsap,
    SendBetween,
    SendAt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationWorkflowTriggerSettingsRuntimeHoursType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SendAsap => serializer.serialize_str("send_asap"),
            Self::SendBetween => serializer.serialize_str("send_between"),
            Self::SendAt => serializer.serialize_str("send_at"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationWorkflowTriggerSettingsRuntimeHoursType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "send_asap" => Ok(Self::SendAsap),
            "send_between" => Ok(Self::SendBetween),
            "send_at" => Ok(Self::SendAt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationWorkflowTriggerSettingsRuntimeHoursType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SendAsap => write!(f, "send_asap"),
            Self::SendBetween => write!(f, "send_between"),
            Self::SendAt => write!(f, "send_at"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
