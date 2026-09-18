pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The predicted age to segment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemPredictedAgeValue {
    Eighteen24,
    TwentyFive34,
    ThirtyFive44,
    FortyFive54,
    FiftyFive64,
    SixtyFive,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SegmentTypeItemPredictedAgeValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eighteen24 => serializer.serialize_str("18-24"),
            Self::TwentyFive34 => serializer.serialize_str("25-34"),
            Self::ThirtyFive44 => serializer.serialize_str("35-44"),
            Self::FortyFive54 => serializer.serialize_str("45-54"),
            Self::FiftyFive64 => serializer.serialize_str("55-64"),
            Self::SixtyFive => serializer.serialize_str("65+"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SegmentTypeItemPredictedAgeValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "18-24" => Ok(Self::Eighteen24),
            "25-34" => Ok(Self::TwentyFive34),
            "35-44" => Ok(Self::ThirtyFive44),
            "45-54" => Ok(Self::FortyFive54),
            "55-64" => Ok(Self::FiftyFive64),
            "65+" => Ok(Self::SixtyFive),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SegmentTypeItemPredictedAgeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eighteen24 => write!(f, "18-24"),
            Self::TwentyFive34 => write!(f, "25-34"),
            Self::ThirtyFive44 => write!(f, "35-44"),
            Self::FortyFive54 => write!(f, "45-54"),
            Self::FiftyFive64 => write!(f, "55-64"),
            Self::SixtyFive => write!(f, "65+"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
