pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Members who have have a rating that is/not exactly a given number or members who have a rating greater/less than a given number.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemMemberRatingOp {
    Is,
    Not,
    Greater,
    Less,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemMemberRatingOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Is => serializer.serialize_str("is"),
            Self::Not => serializer.serialize_str("not"),
            Self::Greater => serializer.serialize_str("greater"),
            Self::Less => serializer.serialize_str("less"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemMemberRatingOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "is" => Ok(Self::Is),
            "not" => Ok(Self::Not),
            "greater" => Ok(Self::Greater),
            "less" => Ok(Self::Less),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemMemberRatingOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Is => write!(f, "is"),
            Self::Not => write!(f, "not"),
            Self::Greater => write!(f, "greater"),
            Self::Less => write!(f, "less"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
