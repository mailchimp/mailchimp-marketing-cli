pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListCampaignsRequestStatus {
    Save,
    Paused,
    Schedule,
    Sending,
    Sent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListCampaignsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Save => serializer.serialize_str("save"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Schedule => serializer.serialize_str("schedule"),
            Self::Sending => serializer.serialize_str("sending"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListCampaignsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "save" => Ok(Self::Save),
            "paused" => Ok(Self::Paused),
            "schedule" => Ok(Self::Schedule),
            "sending" => Ok(Self::Sending),
            "sent" => Ok(Self::Sent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListCampaignsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Save => write!(f, "save"),
            Self::Paused => write!(f, "paused"),
            Self::Schedule => write!(f, "schedule"),
            Self::Sending => write!(f, "sending"),
            Self::Sent => write!(f, "sent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
