pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by influence rating in Social Profiles data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialInfluenceField {
    #[serde(rename = "social_influence")]
    SocialInfluence,
}
impl fmt::Display for SegmentTypeItemSocialInfluenceField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SocialInfluence => "social_influence",
        };
        write!(f, "{}", s)
    }
}
