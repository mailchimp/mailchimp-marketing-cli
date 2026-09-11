pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Members who are/not the exact criteria listed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemPredictedAgeOp {
    #[serde(rename = "is")]
    Is,
}
impl fmt::Display for SegmentTypeItemPredictedAgeOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Is => "is",
        };
        write!(f, "{}", s)
    }
}
