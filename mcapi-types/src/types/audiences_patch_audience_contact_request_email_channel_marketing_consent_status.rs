pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatchAudienceContactRequestEmailChannelMarketingConsentStatus {
    Consented,
    Denied,
    Confirmed,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PatchAudienceContactRequestEmailChannelMarketingConsentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Consented => serializer.serialize_str("consented"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::Confirmed => serializer.serialize_str("confirmed"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PatchAudienceContactRequestEmailChannelMarketingConsentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "consented" => Ok(Self::Consented),
            "denied" => Ok(Self::Denied),
            "confirmed" => Ok(Self::Confirmed),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PatchAudienceContactRequestEmailChannelMarketingConsentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Consented => write!(f, "consented"),
            Self::Denied => write!(f, "denied"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
