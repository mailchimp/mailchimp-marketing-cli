pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of the member with regard to their campaign interaction. One of the following: opened, clicked, was sent, didn't open, didn't click, or was not sent.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemAimOp {
    Open,
    Click,
    Sent,
    Noopen,
    Noclick,
    Nosent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemAimOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Click => serializer.serialize_str("click"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Noopen => serializer.serialize_str("noopen"),
            Self::Noclick => serializer.serialize_str("noclick"),
            Self::Nosent => serializer.serialize_str("nosent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemAimOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "click" => Ok(Self::Click),
            "sent" => Ok(Self::Sent),
            "noopen" => Ok(Self::Noopen),
            "noclick" => Ok(Self::Noclick),
            "nosent" => Ok(Self::Nosent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemAimOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Click => write!(f, "click"),
            Self::Sent => write!(f, "sent"),
            Self::Noopen => write!(f, "noopen"),
            Self::Noclick => write!(f, "noclick"),
            Self::Nosent => write!(f, "nosent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
