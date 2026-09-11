pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segmenting based off of a subscriber's email address.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEmailAddressField {
    Merge0,
    Email,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemEmailAddressField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Merge0 => serializer.serialize_str("merge0"),
            Self::Email => serializer.serialize_str("EMAIL"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemEmailAddressField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "merge0" => Ok(Self::Merge0),
            "EMAIL" => Ok(Self::Email),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemEmailAddressField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Merge0 => write!(f, "merge0"),
            Self::Email => write!(f, "EMAIL"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
