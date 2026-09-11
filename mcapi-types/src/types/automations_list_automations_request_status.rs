pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAutomationsRequestStatus {
    Save,
    Paused,
    Sending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAutomationsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Save => serializer.serialize_str("save"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Sending => serializer.serialize_str("sending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAutomationsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "save" => Ok(Self::Save),
            "paused" => Ok(Self::Paused),
            "sending" => Ok(Self::Sending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAutomationsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Save => write!(f, "save"),
            Self::Paused => write!(f, "paused"),
            Self::Sending => write!(f, "sending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
