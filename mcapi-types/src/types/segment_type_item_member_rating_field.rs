pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by member rating.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemMemberRatingField {
    #[serde(rename = "rating")]
    Rating,
}
impl fmt::Display for SegmentTypeItemMemberRatingField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Rating => "rating",
        };
        write!(f, "{}", s)
    }
}
