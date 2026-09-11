pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment members who are/are not within a specific US zip code.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemIpGeoZipOp {
    Ipgeoiszip,
    Ipgeonotzip,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemIpGeoZipOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ipgeoiszip => serializer.serialize_str("ipgeoiszip"),
            Self::Ipgeonotzip => serializer.serialize_str("ipgeonotzip"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemIpGeoZipOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ipgeoiszip" => Ok(Self::Ipgeoiszip),
            "ipgeonotzip" => Ok(Self::Ipgeonotzip),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemIpGeoZipOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ipgeoiszip => write!(f, "ipgeoiszip"),
            Self::Ipgeonotzip => write!(f, "ipgeonotzip"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
