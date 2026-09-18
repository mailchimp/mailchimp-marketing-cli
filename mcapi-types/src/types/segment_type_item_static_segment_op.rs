pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Members who are/are not apart of a static segment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemStaticSegmentOp {
    StaticIs,
    StaticNot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemStaticSegmentOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StaticIs => serializer.serialize_str("static_is"),
            Self::StaticNot => serializer.serialize_str("static_not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemStaticSegmentOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "static_is" => Ok(Self::StaticIs),
            "static_not" => Ok(Self::StaticNot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemStaticSegmentOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaticIs => write!(f, "static_is"),
            Self::StaticNot => write!(f, "static_not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
