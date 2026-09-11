pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// For A/B Split Campaigns, the group the member was apart of.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SentToAbsplitGroup {
    A,
    B,
    Winner,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SentToAbsplitGroup {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::A => serializer.serialize_str("a"),
            Self::B => serializer.serialize_str("b"),
            Self::Winner => serializer.serialize_str("winner"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SentToAbsplitGroup {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "winner" => Ok(Self::Winner),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SentToAbsplitGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "a"),
            Self::B => write!(f, "b"),
            Self::Winner => write!(f, "winner"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
