pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source of the feedback.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListFeedbackResponseFeedbackItemSource {
    Api,
    Email,
    Sms,
    Web,
    Ios,
    Android,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListFeedbackResponseFeedbackItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Api => serializer.serialize_str("api"),
            Self::Email => serializer.serialize_str("email"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::Web => serializer.serialize_str("web"),
            Self::Ios => serializer.serialize_str("ios"),
            Self::Android => serializer.serialize_str("android"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListFeedbackResponseFeedbackItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api" => Ok(Self::Api),
            "email" => Ok(Self::Email),
            "sms" => Ok(Self::Sms),
            "web" => Ok(Self::Web),
            "ios" => Ok(Self::Ios),
            "android" => Ok(Self::Android),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListFeedbackResponseFeedbackItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::Email => write!(f, "email"),
            Self::Sms => write!(f, "sms"),
            Self::Web => write!(f, "web"),
            Self::Ios => write!(f, "ios"),
            Self::Android => write!(f, "android"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
