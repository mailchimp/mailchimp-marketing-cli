pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by purchases in specific items or categories.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemEcommCategoryField {
    EcommCat,
    EcommProd,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemEcommCategoryField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EcommCat => serializer.serialize_str("ecomm_cat"),
            Self::EcommProd => serializer.serialize_str("ecomm_prod"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemEcommCategoryField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ecomm_cat" => Ok(Self::EcommCat),
            "ecomm_prod" => Ok(Self::EcommProd),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemEcommCategoryField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EcommCat => write!(f, "ecomm_cat"),
            Self::EcommProd => write!(f, "ecomm_prod"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
