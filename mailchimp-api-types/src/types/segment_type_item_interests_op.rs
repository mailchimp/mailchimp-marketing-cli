pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Whether the member is a part of one, all, or none of the groups.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemInterestsOp {
    Interestcontains,
    Interestcontainsall,
    Interestnotcontains,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemInterestsOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Interestcontains => serializer.serialize_str("interestcontains"),
            Self::Interestcontainsall => serializer.serialize_str("interestcontainsall"),
            Self::Interestnotcontains => serializer.serialize_str("interestnotcontains"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemInterestsOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "interestcontains" => Ok(Self::Interestcontains),
            "interestcontainsall" => Ok(Self::Interestcontainsall),
            "interestnotcontains" => Ok(Self::Interestnotcontains),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemInterestsOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Interestcontains => write!(f, "interestcontains"),
            Self::Interestcontainsall => write!(f, "interestcontainsall"),
            Self::Interestnotcontains => write!(f, "interestnotcontains"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
