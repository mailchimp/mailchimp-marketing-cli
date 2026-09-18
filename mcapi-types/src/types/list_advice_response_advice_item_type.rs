pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The sentiment type for a feedback message.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListAdviceResponseAdviceItemType {
    Negative,
    Positive,
    Neutral,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListAdviceResponseAdviceItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Negative => serializer.serialize_str("negative"),
            Self::Positive => serializer.serialize_str("positive"),
            Self::Neutral => serializer.serialize_str("neutral"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListAdviceResponseAdviceItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "negative" => Ok(Self::Negative),
            "positive" => Ok(Self::Positive),
            "neutral" => Ok(Self::Neutral),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListAdviceResponseAdviceItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Negative => write!(f, "negative"),
            Self::Positive => write!(f, "positive"),
            Self::Neutral => write!(f, "neutral"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
