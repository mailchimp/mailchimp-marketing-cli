pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by social network in Social Profiles data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialNetworkMemberField {
    #[serde(rename = "social_network")]
    SocialNetwork,
}
impl fmt::Display for SegmentTypeItemSocialNetworkMemberField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SocialNetwork => "social_network",
        };
        write!(f, "{}", s)
    }
}
