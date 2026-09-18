pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListClickDetailsRequestSortField {
    TotalClicks,
    UniqueClicks,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListClickDetailsRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TotalClicks => serializer.serialize_str("total_clicks"),
            Self::UniqueClicks => serializer.serialize_str("unique_clicks"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListClickDetailsRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "total_clicks" => Ok(Self::TotalClicks),
            "unique_clicks" => Ok(Self::UniqueClicks),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListClickDetailsRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TotalClicks => write!(f, "total_clicks"),
            Self::UniqueClicks => write!(f, "unique_clicks"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
