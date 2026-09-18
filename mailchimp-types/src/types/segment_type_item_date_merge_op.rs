pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the member's merge information is/is not, is greater/less than a value or is/is not blank.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemDateMergeOp {
    Is,
    Not,
    Less,
    Blank,
    BlankNot,
    Greater,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemDateMergeOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Is => serializer.serialize_str("is"),
            Self::Not => serializer.serialize_str("not"),
            Self::Less => serializer.serialize_str("less"),
            Self::Blank => serializer.serialize_str("blank"),
            Self::BlankNot => serializer.serialize_str("blank_not"),
            Self::Greater => serializer.serialize_str("greater"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemDateMergeOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "is" => Ok(Self::Is),
            "not" => Ok(Self::Not),
            "less" => Ok(Self::Less),
            "blank" => Ok(Self::Blank),
            "blank_not" => Ok(Self::BlankNot),
            "greater" => Ok(Self::Greater),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemDateMergeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Is => write!(f, "is"),
            Self::Not => write!(f, "not"),
            Self::Less => write!(f, "less"),
            Self::Blank => write!(f, "blank"),
            Self::BlankNot => write!(f, "blank_not"),
            Self::Greater => write!(f, "greater"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
