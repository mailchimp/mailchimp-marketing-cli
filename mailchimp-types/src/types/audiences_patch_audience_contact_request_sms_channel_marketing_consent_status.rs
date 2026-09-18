pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The contact's SMS marketing consent status. Use `confirmed` for double opt-in audiences, `consented` for single opt-in audiences. `denied` is accepted on PATCH/PUT only (not POST) and drives an API-initiated unsubscribe; it cannot be used when creating a new contact.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatchAudienceContactRequestSmsChannelMarketingConsentStatus {
    Consented,
    Confirmed,
    Denied,
    Unknown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PatchAudienceContactRequestSmsChannelMarketingConsentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Consented => serializer.serialize_str("consented"),
            Self::Confirmed => serializer.serialize_str("confirmed"),
            Self::Denied => serializer.serialize_str("denied"),
            Self::Unknown => serializer.serialize_str("unknown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PatchAudienceContactRequestSmsChannelMarketingConsentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "consented" => Ok(Self::Consented),
            "confirmed" => Ok(Self::Confirmed),
            "denied" => Ok(Self::Denied),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PatchAudienceContactRequestSmsChannelMarketingConsentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Consented => write!(f, "consented"),
            Self::Confirmed => write!(f, "confirmed"),
            Self::Denied => write!(f, "denied"),
            Self::Unknown => write!(f, "unknown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
