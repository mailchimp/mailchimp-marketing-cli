pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The Domain's current status.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateActionVerifyResponseStatus {
    VerificationInProgress,
    Verified,
    Expired,
    Error,
    AuthenticationInProgress,
    AuthenticationError,
    Authenticated,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateActionVerifyResponseStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::VerificationInProgress => serializer.serialize_str("VERIFICATION_IN_PROGRESS"),
            Self::Verified => serializer.serialize_str("VERIFIED"),
            Self::Expired => serializer.serialize_str("EXPIRED"),
            Self::Error => serializer.serialize_str("ERROR"),
            Self::AuthenticationInProgress => serializer.serialize_str("AUTHENTICATION_IN_PROGRESS"),
            Self::AuthenticationError => serializer.serialize_str("AUTHENTICATION_ERROR"),
            Self::Authenticated => serializer.serialize_str("AUTHENTICATED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateActionVerifyResponseStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "VERIFICATION_IN_PROGRESS" => Ok(Self::VerificationInProgress),
            "VERIFIED" => Ok(Self::Verified),
            "EXPIRED" => Ok(Self::Expired),
            "ERROR" => Ok(Self::Error),
            "AUTHENTICATION_IN_PROGRESS" => Ok(Self::AuthenticationInProgress),
            "AUTHENTICATION_ERROR" => Ok(Self::AuthenticationError),
            "AUTHENTICATED" => Ok(Self::Authenticated),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateActionVerifyResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VerificationInProgress => write!(f, "VERIFICATION_IN_PROGRESS"),
            Self::Verified => write!(f, "VERIFIED"),
            Self::Expired => write!(f, "EXPIRED"),
            Self::Error => write!(f, "ERROR"),
            Self::AuthenticationInProgress => write!(f, "AUTHENTICATION_IN_PROGRESS"),
            Self::AuthenticationError => write!(f, "AUTHENTICATION_ERROR"),
            Self::Authenticated => write!(f, "AUTHENTICATED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
