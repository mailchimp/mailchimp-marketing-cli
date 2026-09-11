pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment members for which location information is unknown.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemIpGeoUnknownOp {
    #[serde(rename = "ipgeounknown")]
    Ipgeounknown,
}
impl fmt::Display for SegmentTypeItemIpGeoUnknownOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ipgeounknown => "ipgeounknown",
        };
        write!(f, "{}", s)
    }
}
