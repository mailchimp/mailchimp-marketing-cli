pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A unique code that identifies this specifc error.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BatchSubscribeOrUnsubscribeResponseErrorsItemErrorCode {
    ErrorContactExists,
    ErrorGeneric,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BatchSubscribeOrUnsubscribeResponseErrorsItemErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ErrorContactExists => serializer.serialize_str("ERROR_CONTACT_EXISTS"),
            Self::ErrorGeneric => serializer.serialize_str("ERROR_GENERIC"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BatchSubscribeOrUnsubscribeResponseErrorsItemErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ERROR_CONTACT_EXISTS" => Ok(Self::ErrorContactExists),
            "ERROR_GENERIC" => Ok(Self::ErrorGeneric),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BatchSubscribeOrUnsubscribeResponseErrorsItemErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ErrorContactExists => write!(f, "ERROR_CONTACT_EXISTS"),
            Self::ErrorGeneric => write!(f, "ERROR_GENERIC"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
