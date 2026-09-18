pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whe the event took place, namely within a time frame.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemNewSubscribersOp {
    #[serde(rename = "date_within")]
    DateWithin,
}
impl fmt::Display for SegmentTypeItemNewSubscribersOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::DateWithin => "date_within",
        };
        write!(f, "{}", s)
    }
}
