pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Status of a contacts Marketing Consent
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAudienceContactRequestEmailChannelMarketingConsentStatus {
    Confirmed,
    Consented,
    Denied,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAudienceContactRequestEmailChannelMarketingConsentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Confirmed => serializer.serialize_str("confirmed"),
            Self::Consented => serializer.serialize_str("consented"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAudienceContactRequestEmailChannelMarketingConsentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "confirmed" => Ok(Self::Confirmed),
            "consented" => Ok(Self::Consented),
            "denied" => Ok(Self::Denied),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAudienceContactRequestEmailChannelMarketingConsentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Confirmed => write!(f, "confirmed"),
            Self::Consented => write!(f, "consented"),
            Self::Denied => write!(f, "denied"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
