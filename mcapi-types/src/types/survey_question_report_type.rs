pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The response type of the survey question.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SurveyQuestionReportType {
    PickOne,
    PickMany,
    Range,
    Text,
    Email,
    ContactInformation,
    Dropdown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SurveyQuestionReportType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PickOne => serializer.serialize_str("pickOne"),
            Self::PickMany => serializer.serialize_str("pickMany"),
            Self::Range => serializer.serialize_str("range"),
            Self::Text => serializer.serialize_str("text"),
            Self::Email => serializer.serialize_str("email"),
            Self::ContactInformation => serializer.serialize_str("contactInformation"),
            Self::Dropdown => serializer.serialize_str("dropdown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SurveyQuestionReportType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pickOne" => Ok(Self::PickOne),
            "pickMany" => Ok(Self::PickMany),
            "range" => Ok(Self::Range),
            "text" => Ok(Self::Text),
            "email" => Ok(Self::Email),
            "contactInformation" => Ok(Self::ContactInformation),
            "dropdown" => Ok(Self::Dropdown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SurveyQuestionReportType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PickOne => write!(f, "pickOne"),
            Self::PickMany => write!(f, "pickMany"),
            Self::Range => write!(f, "range"),
            Self::Text => write!(f, "text"),
            Self::Email => write!(f, "email"),
            Self::ContactInformation => write!(f, "contactInformation"),
            Self::Dropdown => write!(f, "dropdown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
