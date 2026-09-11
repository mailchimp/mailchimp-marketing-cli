pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by a given static segment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemStaticSegmentField {
    #[serde(rename = "static_segment")]
    StaticSegment,
}
impl fmt::Display for SegmentTypeItemStaticSegmentField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::StaticSegment => "static_segment",
        };
        write!(f, "{}", s)
    }
}
