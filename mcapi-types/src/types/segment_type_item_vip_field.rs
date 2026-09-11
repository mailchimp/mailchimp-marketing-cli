pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by VIP status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemVipField {
    #[serde(rename = "gmonkey")]
    Gmonkey,
}
impl fmt::Display for SegmentTypeItemVipField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Gmonkey => "gmonkey",
        };
        write!(f, "{}", s)
    }
}
