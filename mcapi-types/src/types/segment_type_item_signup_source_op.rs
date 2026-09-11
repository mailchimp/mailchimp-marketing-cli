pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the member's signup source was/was not a particular value.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSignupSourceOp {
    SourceIs,
    SourceNot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemSignupSourceOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SourceIs => serializer.serialize_str("source_is"),
            Self::SourceNot => serializer.serialize_str("source_not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemSignupSourceOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "source_is" => Ok(Self::SourceIs),
            "source_not" => Ok(Self::SourceNot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemSignupSourceOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceIs => write!(f, "source_is"),
            Self::SourceNot => write!(f, "source_not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
