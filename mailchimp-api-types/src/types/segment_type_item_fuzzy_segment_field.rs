pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by similar subscribers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemFuzzySegmentField {
    #[serde(rename = "fuzzy_segment")]
    FuzzySegment,
}
impl fmt::Display for SegmentTypeItemFuzzySegmentField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::FuzzySegment => "fuzzy_segment",
        };
        write!(f, "{}", s)
    }
}
