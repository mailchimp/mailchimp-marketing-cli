pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by listed gender in Social Profiles data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialGenderField {
    #[serde(rename = "social_gender")]
    SocialGender,
}
impl fmt::Display for SegmentTypeItemSocialGenderField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::SocialGender => "social_gender",
        };
        write!(f, "{}", s)
    }
}
