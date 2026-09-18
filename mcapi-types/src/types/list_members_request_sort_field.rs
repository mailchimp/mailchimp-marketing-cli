pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMembersRequestSortField {
    TimestampOpt,
    TimestampSignup,
    LastChanged,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMembersRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TimestampOpt => serializer.serialize_str("timestamp_opt"),
            Self::TimestampSignup => serializer.serialize_str("timestamp_signup"),
            Self::LastChanged => serializer.serialize_str("last_changed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMembersRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "timestamp_opt" => Ok(Self::TimestampOpt),
            "timestamp_signup" => Ok(Self::TimestampSignup),
            "last_changed" => Ok(Self::LastChanged),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMembersRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimestampOpt => write!(f, "timestamp_opt"),
            Self::TimestampSignup => write!(f, "timestamp_signup"),
            Self::LastChanged => write!(f, "last_changed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
