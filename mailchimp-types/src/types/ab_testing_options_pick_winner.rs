pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How we should evaluate a winner. Based on 'opens', 'clicks', or 'manual'.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbTestingOptionsPickWinner {
    Opens,
    Clicks,
    Manual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AbTestingOptionsPickWinner {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Opens => serializer.serialize_str("opens"),
            Self::Clicks => serializer.serialize_str("clicks"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AbTestingOptionsPickWinner {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "opens" => Ok(Self::Opens),
            "clicks" => Ok(Self::Clicks),
            "manual" => Ok(Self::Manual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AbTestingOptionsPickWinner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Opens => write!(f, "opens"),
            Self::Clicks => write!(f, "clicks"),
            Self::Manual => write!(f, "manual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
