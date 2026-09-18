pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudiencesContactEmailChannelEffectiveSubscriptionStatusValue {
    Subscribed,
    Unsubscribed,
    Nonsubscribed,
    Pending,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudiencesContactEmailChannelEffectiveSubscriptionStatusValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Subscribed => serializer.serialize_str("subscribed"),
            Self::Unsubscribed => serializer.serialize_str("unsubscribed"),
            Self::Nonsubscribed => serializer.serialize_str("nonsubscribed"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudiencesContactEmailChannelEffectiveSubscriptionStatusValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "subscribed" => Ok(Self::Subscribed),
            "unsubscribed" => Ok(Self::Unsubscribed),
            "nonsubscribed" => Ok(Self::Nonsubscribed),
            "pending" => Ok(Self::Pending),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudiencesContactEmailChannelEffectiveSubscriptionStatusValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Subscribed => write!(f, "subscribed"),
            Self::Unsubscribed => write!(f, "unsubscribed"),
            Self::Nonsubscribed => write!(f, "nonsubscribed"),
            Self::Pending => write!(f, "pending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
