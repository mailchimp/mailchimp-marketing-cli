pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which campaign resend shortcut to use. Default is `to_non_openers`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateActionCreateResendCampaignsRequestShortcutType {
    ToNonOpeners,
    ToNewSubscribers,
    ToNonClickers,
    ToNonPurchasers,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateActionCreateResendCampaignsRequestShortcutType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ToNonOpeners => serializer.serialize_str("to_non_openers"),
            Self::ToNewSubscribers => serializer.serialize_str("to_new_subscribers"),
            Self::ToNonClickers => serializer.serialize_str("to_non_clickers"),
            Self::ToNonPurchasers => serializer.serialize_str("to_non_purchasers"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateActionCreateResendCampaignsRequestShortcutType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "to_non_openers" => Ok(Self::ToNonOpeners),
            "to_new_subscribers" => Ok(Self::ToNewSubscribers),
            "to_non_clickers" => Ok(Self::ToNonClickers),
            "to_non_purchasers" => Ok(Self::ToNonPurchasers),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateActionCreateResendCampaignsRequestShortcutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToNonOpeners => write!(f, "to_non_openers"),
            Self::ToNewSubscribers => write!(f, "to_new_subscribers"),
            Self::ToNonClickers => write!(f, "to_non_clickers"),
            Self::ToNonPurchasers => write!(f, "to_non_purchasers"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
