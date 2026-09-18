pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Members who are/not following a linked account on a given social network.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialNetworkFollowOp {
    Follow,
    Notfollow,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemSocialNetworkFollowOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Follow => serializer.serialize_str("follow"),
            Self::Notfollow => serializer.serialize_str("notfollow"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemSocialNetworkFollowOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "follow" => Ok(Self::Follow),
            "notfollow" => Ok(Self::Notfollow),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemSocialNetworkFollowOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Follow => write!(f, "follow"),
            Self::Notfollow => write!(f, "notfollow"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
