pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by interaction with an Automation workflow.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemAutomationField {
    #[serde(rename = "automation")]
    Automation,
}
impl fmt::Display for SegmentTypeItemAutomationField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Automation => "automation",
        };
        write!(f, "{}", s)
    }
}
