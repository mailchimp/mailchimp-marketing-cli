pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The survey's status.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetSurveyResponseStatus {
    Published,
    Unpublished,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetSurveyResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Published => serializer.serialize_str("published"),
            Self::Unpublished => serializer.serialize_str("unpublished"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetSurveyResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "published" => Ok(Self::Published),
            "unpublished" => Ok(Self::Unpublished),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetSurveyResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Published => write!(f, "published"),
            Self::Unpublished => write!(f, "unpublished"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
