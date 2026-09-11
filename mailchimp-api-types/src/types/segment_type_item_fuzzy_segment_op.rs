pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Members who are/are not apart of a 'similar subscribers' segment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemFuzzySegmentOp {
    FuzzyIs,
    FuzzyNot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemFuzzySegmentOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FuzzyIs => serializer.serialize_str("fuzzy_is"),
            Self::FuzzyNot => serializer.serialize_str("fuzzy_not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemFuzzySegmentOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "fuzzy_is" => Ok(Self::FuzzyIs),
            "fuzzy_not" => Ok(Self::FuzzyNot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemFuzzySegmentOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FuzzyIs => write!(f, "fuzzy_is"),
            Self::FuzzyNot => write!(f, "fuzzy_not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
