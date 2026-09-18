pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The section type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SurveySectionRequestType {
    Introduction,
    Context,
    Question,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SurveySectionRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Introduction => serializer.serialize_str("introduction"),
            Self::Context => serializer.serialize_str("context"),
            Self::Question => serializer.serialize_str("question"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SurveySectionRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "introduction" => Ok(Self::Introduction),
            "context" => Ok(Self::Context),
            "question" => Ok(Self::Question),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SurveySectionRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Introduction => write!(f, "introduction"),
            Self::Context => write!(f, "context"),
            Self::Question => write!(f, "question"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
