pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of this landing page.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LandingPageStatus {
    Published,
    Unpublished,
    Draft,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LandingPageStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Published => serializer.serialize_str("published"),
            Self::Unpublished => serializer.serialize_str("unpublished"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LandingPageStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "published" => Ok(Self::Published),
            "unpublished" => Ok(Self::Unpublished),
            "draft" => Ok(Self::Draft),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LandingPageStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Published => write!(f, "published"),
            Self::Unpublished => write!(f, "unpublished"),
            Self::Draft => write!(f, "draft"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
