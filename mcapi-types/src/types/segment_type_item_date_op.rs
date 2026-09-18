pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// When the event took place:  Before, after, is a specific date, is not a specific date, is blank, or is not blank.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemDateOp {
    Greater,
    Less,
    Is,
    Not,
    Blank,
    BlankNot,
    Within,
    Notwithin,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemDateOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Greater => serializer.serialize_str("greater"),
            Self::Less => serializer.serialize_str("less"),
            Self::Is => serializer.serialize_str("is"),
            Self::Not => serializer.serialize_str("not"),
            Self::Blank => serializer.serialize_str("blank"),
            Self::BlankNot => serializer.serialize_str("blank_not"),
            Self::Within => serializer.serialize_str("within"),
            Self::Notwithin => serializer.serialize_str("notwithin"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemDateOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "greater" => Ok(Self::Greater),
            "less" => Ok(Self::Less),
            "is" => Ok(Self::Is),
            "not" => Ok(Self::Not),
            "blank" => Ok(Self::Blank),
            "blank_not" => Ok(Self::BlankNot),
            "within" => Ok(Self::Within),
            "notwithin" => Ok(Self::Notwithin),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemDateOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Greater => write!(f, "greater"),
            Self::Less => write!(f, "less"),
            Self::Is => write!(f, "is"),
            Self::Not => write!(f, "not"),
            Self::Blank => write!(f, "blank"),
            Self::BlankNot => write!(f, "blank_not"),
            Self::Within => write!(f, "within"),
            Self::Notwithin => write!(f, "notwithin"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
