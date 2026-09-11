pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of activity
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListChimpChatterActivityFeedResponseChimpChatterItemType {
    ListsNewSubscriber,
    ListsUnsubscribes,
    ListsProfileUpdates,
    CampaignsFacebookLikes,
    CampaignsForwardToFriend,
    ListsImports,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListChimpChatterActivityFeedResponseChimpChatterItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ListsNewSubscriber => serializer.serialize_str("lists:new-subscriber"),
            Self::ListsUnsubscribes => serializer.serialize_str("lists:unsubscribes"),
            Self::ListsProfileUpdates => serializer.serialize_str("lists:profile-updates"),
            Self::CampaignsFacebookLikes => serializer.serialize_str("campaigns:facebook-likes"),
            Self::CampaignsForwardToFriend => serializer.serialize_str("campaigns:forward-to-friend"),
            Self::ListsImports => serializer.serialize_str("lists:imports"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListChimpChatterActivityFeedResponseChimpChatterItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "lists:new-subscriber" => Ok(Self::ListsNewSubscriber),
            "lists:unsubscribes" => Ok(Self::ListsUnsubscribes),
            "lists:profile-updates" => Ok(Self::ListsProfileUpdates),
            "campaigns:facebook-likes" => Ok(Self::CampaignsFacebookLikes),
            "campaigns:forward-to-friend" => Ok(Self::CampaignsForwardToFriend),
            "lists:imports" => Ok(Self::ListsImports),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListChimpChatterActivityFeedResponseChimpChatterItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListsNewSubscriber => write!(f, "lists:new-subscriber"),
            Self::ListsUnsubscribes => write!(f, "lists:unsubscribes"),
            Self::ListsProfileUpdates => write!(f, "lists:profile-updates"),
            Self::CampaignsFacebookLikes => write!(f, "campaigns:facebook-likes"),
            Self::CampaignsForwardToFriend => write!(f, "campaigns:forward-to-friend"),
            Self::ListsImports => write!(f, "lists:imports"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
