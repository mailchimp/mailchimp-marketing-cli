pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Subscriber's current status.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListsPostStatus {
    Subscribed,
    Unsubscribed,
    Cleaned,
    Pending,
    Transactional,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListsPostStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Subscribed => serializer.serialize_str("subscribed"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::Cleaned => serializer.serialize_str("cleaned"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Transactional => serializer.serialize_str("transactional"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListsPostStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "subscribed" => Ok(Self::Subscribed),
            "unsubscribed" => Ok(Self::Unsubscribed),
            "cleaned" => Ok(Self::Cleaned),
            "pending" => Ok(Self::Pending),
            "transactional" => Ok(Self::Transactional),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListsPostStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Subscribed => write!(f, "subscribed"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::Cleaned => write!(f, "cleaned"),
            Self::Pending => write!(f, "pending"),
            Self::Transactional => write!(f, "transactional"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
