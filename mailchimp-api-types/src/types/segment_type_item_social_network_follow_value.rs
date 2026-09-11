pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The social network to segment against.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialNetworkFollowValue {
    #[serde(rename = "twitter_follow")]
    TwitterFollow,
}
impl fmt::Display for SegmentTypeItemSocialNetworkFollowValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TwitterFollow => "twitter_follow",
        };
        write!(f, "{}", s)
    }
}
