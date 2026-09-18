pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by poll activity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemCampaignPollField {
    #[serde(rename = "poll")]
    Poll,
}
impl fmt::Display for SegmentTypeItemCampaignPollField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Poll => "poll",
        };
        write!(f, "{}", s)
    }
}
