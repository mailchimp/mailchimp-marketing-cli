pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The current state of a campaign delivery.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignsDeliveryStatusStatus {
    Delivering,
    Delivered,
    Canceling,
    Canceled,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignsDeliveryStatusStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Delivering => serializer.serialize_str("delivering"),
            Self::Delivered => serializer.serialize_str("delivered"),
            Self::Canceling => serializer.serialize_str("canceling"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignsDeliveryStatusStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "delivering" => Ok(Self::Delivering),
            "delivered" => Ok(Self::Delivered),
            "canceling" => Ok(Self::Canceling),
            "canceled" => Ok(Self::Canceled),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignsDeliveryStatusStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Delivering => write!(f, "delivering"),
            Self::Delivered => write!(f, "delivered"),
            Self::Canceling => write!(f, "canceling"),
            Self::Canceled => write!(f, "canceled"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
