pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment members who are within a specific country or US state.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemIpGeoCountryStateOp {
    Ipgeocountry,
    Ipgeonotcountry,
    Ipgeostate,
    Ipgeonotstate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemIpGeoCountryStateOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ipgeocountry => serializer.serialize_str("ipgeocountry"),
            Self::Ipgeonotcountry => serializer.serialize_str("ipgeonotcountry"),
            Self::Ipgeostate => serializer.serialize_str("ipgeostate"),
            Self::Ipgeonotstate => serializer.serialize_str("ipgeonotstate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemIpGeoCountryStateOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ipgeocountry" => Ok(Self::Ipgeocountry),
            "ipgeonotcountry" => Ok(Self::Ipgeonotcountry),
            "ipgeostate" => Ok(Self::Ipgeostate),
            "ipgeonotstate" => Ok(Self::Ipgeonotstate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemIpGeoCountryStateOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ipgeocountry => write!(f, "ipgeocountry"),
            Self::Ipgeonotcountry => write!(f, "ipgeonotcountry"),
            Self::Ipgeostate => write!(f, "ipgeostate"),
            Self::Ipgeonotstate => write!(f, "ipgeonotstate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
