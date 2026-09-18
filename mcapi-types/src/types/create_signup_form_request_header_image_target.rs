pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Image link target.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateSignupFormRequestHeaderImageTarget {
    Blank,
    Null,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateSignupFormRequestHeaderImageTarget {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Blank => serializer.serialize_str("_blank"),
            Self::Null => serializer.serialize_str("null"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateSignupFormRequestHeaderImageTarget {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "_blank" => Ok(Self::Blank),
            "null" => Ok(Self::Null),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateSignupFormRequestHeaderImageTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => write!(f, "_blank"),
            Self::Null => write!(f, "null"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
