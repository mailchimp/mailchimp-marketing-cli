pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of the email delivered to this subscriber. `hard` and `soft` refer to different [bounce types](https://mailchimp.com/help/soft-vs-hard-bounces/).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SentToStatus {
    Sent,
    Hard,
    Soft,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SentToStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sent => serializer.serialize_str("sent"),
            Self::Hard => serializer.serialize_str("hard"),
            Self::Soft => serializer.serialize_str("soft"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SentToStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sent" => Ok(Self::Sent),
            "hard" => Ok(Self::Hard),
            "soft" => Ok(Self::Soft),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SentToStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::Hard => write!(f, "hard"),
            Self::Soft => write!(f, "soft"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
