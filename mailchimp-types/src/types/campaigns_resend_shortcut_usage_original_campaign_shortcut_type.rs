pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Which campaign resend shortcut was used.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignsResendShortcutUsageOriginalCampaignShortcutType {
    NonOpeners,
    NewSubscribers,
    NonClickers,
    NonPurchasers,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignsResendShortcutUsageOriginalCampaignShortcutType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::NonOpeners => serializer.serialize_str("non_openers"),
            Self::NewSubscribers => serializer.serialize_str("new_subscribers"),
            Self::NonClickers => serializer.serialize_str("non_clickers"),
            Self::NonPurchasers => serializer.serialize_str("non_purchasers"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignsResendShortcutUsageOriginalCampaignShortcutType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "non_openers" => Ok(Self::NonOpeners),
            "new_subscribers" => Ok(Self::NewSubscribers),
            "non_clickers" => Ok(Self::NonClickers),
            "non_purchasers" => Ok(Self::NonPurchasers),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignsResendShortcutUsageOriginalCampaignShortcutType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonOpeners => write!(f, "non_openers"),
            Self::NewSubscribers => write!(f, "new_subscribers"),
            Self::NonClickers => write!(f, "non_clickers"),
            Self::NonPurchasers => write!(f, "non_purchasers"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
