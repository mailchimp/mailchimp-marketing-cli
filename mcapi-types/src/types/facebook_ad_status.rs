pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status of this outreach.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FacebookAdStatus {
    Save,
    Paused,
    Schedule,
    Scheduled,
    Sending,
    Sent,
    Canceled,
    Canceling,
    Active,
    Disconnected,
    Somepaused,
    Draft,
    Completed,
    PartialRejected,
    Pending,
    Rejected,
    Published,
    Unpublished,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FacebookAdStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Save => serializer.serialize_str("save"),
            Self::Paused => serializer.serialize_str("paused"),
            Self::Schedule => serializer.serialize_str("schedule"),
            Self::Scheduled => serializer.serialize_str("scheduled"),
            Self::Sending => serializer.serialize_str("sending"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::Canceling => serializer.serialize_str("canceling"),
            Self::Active => serializer.serialize_str("active"),
            Self::Disconnected => serializer.serialize_str("disconnected"),
            Self::Somepaused => serializer.serialize_str("somepaused"),
            Self::Draft => serializer.serialize_str("draft"),
            Self::Completed => serializer.serialize_str("completed"),
            Self::PartialRejected => serializer.serialize_str("partialRejected"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Rejected => serializer.serialize_str("rejected"),
            Self::Published => serializer.serialize_str("published"),
            Self::Unpublished => serializer.serialize_str("unpublished"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FacebookAdStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "save" => Ok(Self::Save),
            "paused" => Ok(Self::Paused),
            "schedule" => Ok(Self::Schedule),
            "scheduled" => Ok(Self::Scheduled),
            "sending" => Ok(Self::Sending),
            "sent" => Ok(Self::Sent),
            "canceled" => Ok(Self::Canceled),
            "canceling" => Ok(Self::Canceling),
            "active" => Ok(Self::Active),
            "disconnected" => Ok(Self::Disconnected),
            "somepaused" => Ok(Self::Somepaused),
            "draft" => Ok(Self::Draft),
            "completed" => Ok(Self::Completed),
            "partialRejected" => Ok(Self::PartialRejected),
            "pending" => Ok(Self::Pending),
            "rejected" => Ok(Self::Rejected),
            "published" => Ok(Self::Published),
            "unpublished" => Ok(Self::Unpublished),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FacebookAdStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Save => write!(f, "save"),
            Self::Paused => write!(f, "paused"),
            Self::Schedule => write!(f, "schedule"),
            Self::Scheduled => write!(f, "scheduled"),
            Self::Sending => write!(f, "sending"),
            Self::Sent => write!(f, "sent"),
            Self::Canceled => write!(f, "canceled"),
            Self::Canceling => write!(f, "canceling"),
            Self::Active => write!(f, "active"),
            Self::Disconnected => write!(f, "disconnected"),
            Self::Somepaused => write!(f, "somepaused"),
            Self::Draft => write!(f, "draft"),
            Self::Completed => write!(f, "completed"),
            Self::PartialRejected => write!(f, "partialRejected"),
            Self::Pending => write!(f, "pending"),
            Self::Rejected => write!(f, "rejected"),
            Self::Published => write!(f, "published"),
            Self::Unpublished => write!(f, "unpublished"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
