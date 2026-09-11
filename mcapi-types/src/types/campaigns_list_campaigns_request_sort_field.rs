pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListCampaignsRequestSortField {
    CreateTime,
    SendTime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListCampaignsRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreateTime => serializer.serialize_str("create_time"),
            Self::SendTime => serializer.serialize_str("send_time"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListCampaignsRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "create_time" => Ok(Self::CreateTime),
            "send_time" => Ok(Self::SendTime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListCampaignsRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateTime => write!(f, "create_time"),
            Self::SendTime => write!(f, "send_time"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
