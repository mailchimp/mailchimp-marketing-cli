pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of date field to segment on: The opt-in time for a signup, the date the subscriber was last updated, or the date of their last ecomm purchase.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemDateField {
    TimestampOpt,
    InfoChanged,
    EcommDate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemDateField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TimestampOpt => serializer.serialize_str("timestamp_opt"),
            Self::InfoChanged => serializer.serialize_str("info_changed"),
            Self::EcommDate => serializer.serialize_str("ecomm_date"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemDateField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "timestamp_opt" => Ok(Self::TimestampOpt),
            "info_changed" => Ok(Self::InfoChanged),
            "ecomm_date" => Ok(Self::EcommDate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemDateField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TimestampOpt => write!(f, "timestamp_opt"),
            Self::InfoChanged => write!(f, "info_changed"),
            Self::EcommDate => write!(f, "ecomm_date"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
