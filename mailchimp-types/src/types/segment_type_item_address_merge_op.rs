pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the member's address merge field contains/does not contain a value or is/is not blank.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemAddressMergeOp {
    Contains,
    Notcontain,
    Blank,
    BlankNot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemAddressMergeOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Contains => serializer.serialize_str("contains"),
            Self::Notcontain => serializer.serialize_str("notcontain"),
            Self::Blank => serializer.serialize_str("blank"),
            Self::BlankNot => serializer.serialize_str("blank_not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemAddressMergeOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "contains" => Ok(Self::Contains),
            "notcontain" => Ok(Self::Notcontain),
            "blank" => Ok(Self::Blank),
            "blank_not" => Ok(Self::BlankNot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemAddressMergeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contains => write!(f, "contains"),
            Self::Notcontain => write!(f, "notcontain"),
            Self::Blank => write!(f, "blank"),
            Self::BlankNot => write!(f, "blank_not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
