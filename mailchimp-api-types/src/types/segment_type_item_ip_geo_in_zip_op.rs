pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment members who are within a specific US zip code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemIpGeoInZipOp {
    #[serde(rename = "ipgeoinzip")]
    Ipgeoinzip,
}
impl fmt::Display for SegmentTypeItemIpGeoInZipOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ipgeoinzip => "ipgeoinzip",
        };
        write!(f, "{}", s)
    }
}
