pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by age ranges in Social Profiles data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialAgeField {
    #[serde(rename = "social_age")]
    SocialAge,
}
impl fmt::Display for SegmentTypeItemSocialAgeField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SocialAge => "social_age",
        };
        write!(f, "{}", s)
    }
}
