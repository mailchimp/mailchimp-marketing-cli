pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the member's address merge field is within a given distance from a city or zip.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemZipMergeOp {
    #[serde(rename = "geoin")]
    Geoin,
}
impl fmt::Display for SegmentTypeItemZipMergeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Geoin => "geoin",
        };
        write!(f, "{}", s)
    }
}
