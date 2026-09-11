pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Status of the abandonedBrowse automation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ECommerceStoreAutomationsAbandonedBrowseStatus {
    Save,
    Sending,
    Paused,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ECommerceStoreAutomationsAbandonedBrowseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Save => serializer.serialize_str("save"),
            Self::Sending => serializer.serialize_str("sending"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ECommerceStoreAutomationsAbandonedBrowseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "save" => Ok(Self::Save),
            "sending" => Ok(Self::Sending),
            "paused" => Ok(Self::Paused),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ECommerceStoreAutomationsAbandonedBrowseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Save => write!(f, "save"),
            Self::Sending => write!(f, "sending"),
            Self::Paused => write!(f, "paused"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
