pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of the batch call. [Learn more](https://mailchimp.com/developer/marketing/guides/run-async-requests-batch-endpoint/#check-the-status-of-a-batch-operation) about the batch operation status.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BatchStatus {
    Pending,
    Preprocessing,
    Started,
    Finalizing,
    Finished,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BatchStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Preprocessing => serializer.serialize_str("preprocessing"),
            Self::Started => serializer.serialize_str("started"),
            Self::Finalizing => serializer.serialize_str("finalizing"),
            Self::Finished => serializer.serialize_str("finished"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BatchStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "preprocessing" => Ok(Self::Preprocessing),
            "started" => Ok(Self::Started),
            "finalizing" => Ok(Self::Finalizing),
            "finished" => Ok(Self::Finished),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Preprocessing => write!(f, "preprocessing"),
            Self::Started => write!(f, "started"),
            Self::Finalizing => write!(f, "finalizing"),
            Self::Finished => write!(f, "finished"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
