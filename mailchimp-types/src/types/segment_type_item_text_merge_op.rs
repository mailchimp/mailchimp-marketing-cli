pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the member's merge information is/is not, contains/does not contain, starts/ends with, or is greater/less than a value
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemTextMergeOp {
    Is,
    Not,
    Contains,
    Notcontain,
    Starts,
    Ends,
    Greater,
    Less,
    Blank,
    BlankNot,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemTextMergeOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Is => serializer.serialize_str("is"),
            Self::Not => serializer.serialize_str("not"),
            Self::Contains => serializer.serialize_str("contains"),
            Self::Notcontain => serializer.serialize_str("notcontain"),
            Self::Starts => serializer.serialize_str("starts"),
            Self::Ends => serializer.serialize_str("ends"),
            Self::Greater => serializer.serialize_str("greater"),
            Self::Less => serializer.serialize_str("less"),
            Self::Blank => serializer.serialize_str("blank"),
            Self::BlankNot => serializer.serialize_str("blank_not"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemTextMergeOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "is" => Ok(Self::Is),
            "not" => Ok(Self::Not),
            "contains" => Ok(Self::Contains),
            "notcontain" => Ok(Self::Notcontain),
            "starts" => Ok(Self::Starts),
            "ends" => Ok(Self::Ends),
            "greater" => Ok(Self::Greater),
            "less" => Ok(Self::Less),
            "blank" => Ok(Self::Blank),
            "blank_not" => Ok(Self::BlankNot),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemTextMergeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Is => write!(f, "is"),
            Self::Not => write!(f, "not"),
            Self::Contains => write!(f, "contains"),
            Self::Notcontain => write!(f, "notcontain"),
            Self::Starts => write!(f, "starts"),
            Self::Ends => write!(f, "ends"),
            Self::Greater => write!(f, "greater"),
            Self::Less => write!(f, "less"),
            Self::Blank => write!(f, "blank"),
            Self::BlankNot => write!(f, "blank_not"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
