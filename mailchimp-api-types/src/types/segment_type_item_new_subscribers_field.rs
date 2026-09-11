pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by when people subscribed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemNewSubscribersField {
    #[serde(rename = "timestamp_opt")]
    TimestampOpt,
}
impl fmt::Display for SegmentTypeItemNewSubscribersField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TimestampOpt => "timestamp_opt",
        };
        write!(f, "{}", s)
    }
}
