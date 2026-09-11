pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Type of the audience
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReportingFacebookAdAudienceType {
    CustomAudience,
    LookalikeAudience,
    InterestBasedAudience,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReportingFacebookAdAudienceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CustomAudience => serializer.serialize_str("Custom Audience"),
            Self::LookalikeAudience => serializer.serialize_str("Lookalike Audience"),
            Self::InterestBasedAudience => serializer.serialize_str("Interest-based Audience"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReportingFacebookAdAudienceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Custom Audience" => Ok(Self::CustomAudience),
            "Lookalike Audience" => Ok(Self::LookalikeAudience),
            "Interest-based Audience" => Ok(Self::InterestBasedAudience),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReportingFacebookAdAudienceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomAudience => write!(f, "Custom Audience"),
            Self::LookalikeAudience => write!(f, "Lookalike Audience"),
            Self::InterestBasedAudience => write!(f, "Interest-based Audience"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
