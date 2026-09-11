pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the member's language is or is not set to a specific language.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemLanguageOp {
    Is,
    Not,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemLanguageOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Is => serializer.serialize_str("is"),
            Self::Not => serializer.serialize_str("not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemLanguageOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "is" => Ok(Self::Is),
            "not" => Ok(Self::Not),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemLanguageOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Is => write!(f, "is"),
            Self::Not => write!(f, "not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
