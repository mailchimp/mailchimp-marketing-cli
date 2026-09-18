pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The contact's current status.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListSurveyResponsesResponseResponsesItemContactStatus {
    Subscribed,
    Unsubscribed,
    NonSubscribed,
    Cleaned,
    Archived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListSurveyResponsesResponseResponsesItemContactStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Subscribed => serializer.serialize_str("Subscribed"),
            Self::Unsubscribed => serializer.serialize_str("Unsubscribed"),
            Self::NonSubscribed => serializer.serialize_str("Non-Subscribed"),
            Self::Cleaned => serializer.serialize_str("Cleaned"),
            Self::Archived => serializer.serialize_str("Archived"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListSurveyResponsesResponseResponsesItemContactStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Subscribed" => Ok(Self::Subscribed),
            "Unsubscribed" => Ok(Self::Unsubscribed),
            "Non-Subscribed" => Ok(Self::NonSubscribed),
            "Cleaned" => Ok(Self::Cleaned),
            "Archived" => Ok(Self::Archived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListSurveyResponsesResponseResponsesItemContactStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Subscribed => write!(f, "Subscribed"),
            Self::Unsubscribed => write!(f, "Unsubscribed"),
            Self::NonSubscribed => write!(f, "Non-Subscribed"),
            Self::Cleaned => write!(f, "Cleaned"),
            Self::Archived => write!(f, "Archived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
