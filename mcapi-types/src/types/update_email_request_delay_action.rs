pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The action that triggers the delay of an automation emails.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateEmailRequestDelayAction {
    Signup,
    EcommAbandonedBrowse,
    EcommAbandonedCart,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateEmailRequestDelayAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Signup => serializer.serialize_str("signup"),
            Self::EcommAbandonedBrowse => serializer.serialize_str("ecomm_abandoned_browse"),
            Self::EcommAbandonedCart => serializer.serialize_str("ecomm_abandoned_cart"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateEmailRequestDelayAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "signup" => Ok(Self::Signup),
            "ecomm_abandoned_browse" => Ok(Self::EcommAbandonedBrowse),
            "ecomm_abandoned_cart" => Ok(Self::EcommAbandonedCart),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateEmailRequestDelayAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signup => write!(f, "signup"),
            Self::EcommAbandonedBrowse => write!(f, "ecomm_abandoned_browse"),
            Self::EcommAbandonedCart => write!(f, "ecomm_abandoned_cart"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
