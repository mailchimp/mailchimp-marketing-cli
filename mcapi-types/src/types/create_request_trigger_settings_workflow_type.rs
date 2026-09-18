pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of Automation workflow.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateRequestTriggerSettingsWorkflowType {
    AbandonedBrowse,
    AbandonedCart,
    EmailFollowup,
    SingleWelcome,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateRequestTriggerSettingsWorkflowType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AbandonedBrowse => serializer.serialize_str("abandonedBrowse"),
            Self::AbandonedCart => serializer.serialize_str("abandonedCart"),
            Self::EmailFollowup => serializer.serialize_str("emailFollowup"),
            Self::SingleWelcome => serializer.serialize_str("singleWelcome"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateRequestTriggerSettingsWorkflowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "abandonedBrowse" => Ok(Self::AbandonedBrowse),
            "abandonedCart" => Ok(Self::AbandonedCart),
            "emailFollowup" => Ok(Self::EmailFollowup),
            "singleWelcome" => Ok(Self::SingleWelcome),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateRequestTriggerSettingsWorkflowType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AbandonedBrowse => write!(f, "abandonedBrowse"),
            Self::AbandonedCart => write!(f, "abandonedCart"),
            Self::EmailFollowup => write!(f, "emailFollowup"),
            Self::SingleWelcome => write!(f, "singleWelcome"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
