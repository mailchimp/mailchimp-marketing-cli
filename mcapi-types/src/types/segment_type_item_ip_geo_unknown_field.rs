pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segmenting subscribers who are within a specific location.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemIpGeoUnknownField {
    #[serde(rename = "ipgeo")]
    Ipgeo,
}
impl fmt::Display for SegmentTypeItemIpGeoUnknownField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ipgeo => "ipgeo",
        };
        write!(f, "{}", s)
    }
}
