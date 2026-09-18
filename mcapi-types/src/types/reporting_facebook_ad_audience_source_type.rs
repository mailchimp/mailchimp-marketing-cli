pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List or Facebook based audience
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReportingFacebookAdAudienceSourceType {
    Facebook,
    List,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReportingFacebookAdAudienceSourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Facebook => serializer.serialize_str("facebook"),
            Self::List => serializer.serialize_str("list"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReportingFacebookAdAudienceSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "facebook" => Ok(Self::Facebook),
            "list" => Ok(Self::List),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReportingFacebookAdAudienceSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Facebook => write!(f, "facebook"),
            Self::List => write!(f, "list"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
