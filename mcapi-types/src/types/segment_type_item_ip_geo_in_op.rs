pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment members who are within a specific geographic region.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemIpGeoInOp {
    Ipgeoin,
    Ipgeonotin,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemIpGeoInOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ipgeoin => serializer.serialize_str("ipgeoin"),
            Self::Ipgeonotin => serializer.serialize_str("ipgeonotin"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemIpGeoInOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ipgeoin" => Ok(Self::Ipgeoin),
            "ipgeonotin" => Ok(Self::Ipgeonotin),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemIpGeoInOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ipgeoin => write!(f, "ipgeoin"),
            Self::Ipgeonotin => write!(f, "ipgeonotin"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
