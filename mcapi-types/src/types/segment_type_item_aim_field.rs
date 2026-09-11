pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by interaction with a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemAimField {
    #[serde(rename = "aim")]
    Aim,
}
impl fmt::Display for SegmentTypeItemAimField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Aim => "aim",
        };
        write!(f, "{}", s)
    }
}
