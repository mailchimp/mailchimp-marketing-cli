pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of the member with regard to the automation workflow. One of the following: has started the workflow, has completed the workflow, has not started the workflow, or has not completed the workflow.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemAutomationOp {
    Started,
    Completed,
    NotStarted,
    NotCompleted,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemAutomationOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Started => serializer.serialize_str("started"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::NotStarted => serializer.serialize_str("not_started"),
            Self::NotCompleted => serializer.serialize_str("not_completed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemAutomationOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "started" => Ok(Self::Started),
            "completed" => Ok(Self::Completed),
            "not_started" => Ok(Self::NotStarted),
            "not_completed" => Ok(Self::NotCompleted),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemAutomationOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Started => write!(f, "started"),
            Self::Completed => write!(f, "completed"),
            Self::NotStarted => write!(f, "not_started"),
            Self::NotCompleted => write!(f, "not_completed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
