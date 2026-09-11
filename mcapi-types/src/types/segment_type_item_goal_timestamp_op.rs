pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the website activity happened after, before, or at a given timestamp.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemGoalTimestampOp {
    Greater,
    Less,
    Is,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemGoalTimestampOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Greater => serializer.serialize_str("greater"),
            Self::Less => serializer.serialize_str("less"),
            Self::Is => serializer.serialize_str("is"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemGoalTimestampOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "greater" => Ok(Self::Greater),
            "less" => Ok(Self::Less),
            "is" => Ok(Self::Is),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemGoalTimestampOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Greater => write!(f, "greater"),
            Self::Less => write!(f, "less"),
            Self::Is => write!(f, "is"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
