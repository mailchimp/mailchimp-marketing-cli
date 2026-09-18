pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The age range to segment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemSocialAgeValue {
    Eighteen24,
    TwentyFive34,
    ThirtyFive54,
    FiftyFive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemSocialAgeValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eighteen24 => serializer.serialize_str("18-24"),
            Self::TwentyFive34 => serializer.serialize_str("25-34"),
            Self::ThirtyFive54 => serializer.serialize_str("35-54"),
            Self::FiftyFive => serializer.serialize_str("55+"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemSocialAgeValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "18-24" => Ok(Self::Eighteen24),
            "25-34" => Ok(Self::TwentyFive34),
            "35-54" => Ok(Self::ThirtyFive54),
            "55+" => Ok(Self::FiftyFive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemSocialAgeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eighteen24 => write!(f, "18-24"),
            Self::TwentyFive34 => write!(f, "25-34"),
            Self::ThirtyFive54 => write!(f, "35-54"),
            Self::FiftyFive => write!(f, "55+"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
