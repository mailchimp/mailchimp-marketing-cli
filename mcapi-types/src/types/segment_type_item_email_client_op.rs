pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The operation to determine whether we select clients that match the value, or clients that do not match the value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEmailClientOp {
    ClientIs,
    ClientNot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemEmailClientOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ClientIs => serializer.serialize_str("client_is"),
            Self::ClientNot => serializer.serialize_str("client_not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemEmailClientOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "client_is" => Ok(Self::ClientIs),
            "client_not" => Ok(Self::ClientNot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemEmailClientOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ClientIs => write!(f, "client_is"),
            Self::ClientNot => write!(f, "client_not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
