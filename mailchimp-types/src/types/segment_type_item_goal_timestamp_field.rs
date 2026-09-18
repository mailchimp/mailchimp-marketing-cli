pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by most recent interaction with a website.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemGoalTimestampField {
    #[serde(rename = "goal_last_visited")]
    GoalLastVisited,
}
impl fmt::Display for SegmentTypeItemGoalTimestampField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::GoalLastVisited => "goal_last_visited",
        };
        write!(f, "{}", s)
    }
}
