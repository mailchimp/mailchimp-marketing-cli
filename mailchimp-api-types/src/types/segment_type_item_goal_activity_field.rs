pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by Goal activity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemGoalActivityField {
    #[serde(rename = "goal")]
    Goal,
}
impl fmt::Display for SegmentTypeItemGoalActivityField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Goal => "goal",
        };
        write!(f, "{}", s)
    }
}
