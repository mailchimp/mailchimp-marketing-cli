pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The content section name.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateSignupFormRequestContentsItemSection {
    SignupMessage,
    UnsubMessage,
    SignupThankYouTitle,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateSignupFormRequestContentsItemSection {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SignupMessage => serializer.serialize_str("signup_message"),
            Self::UnsubMessage => serializer.serialize_str("unsub_message"),
            Self::SignupThankYouTitle => serializer.serialize_str("signup_thank_you_title"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateSignupFormRequestContentsItemSection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "signup_message" => Ok(Self::SignupMessage),
            "unsub_message" => Ok(Self::UnsubMessage),
            "signup_thank_you_title" => Ok(Self::SignupThankYouTitle),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateSignupFormRequestContentsItemSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SignupMessage => write!(f, "signup_message"),
            Self::UnsubMessage => write!(f, "unsub_message"),
            Self::SignupThankYouTitle => write!(f, "signup_thank_you_title"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
