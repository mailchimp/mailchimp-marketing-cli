pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListFacebookAdsReportingRequestSortField {
    CreatedAt,
    UpdatedAt,
    EndTime,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListFacebookAdsReportingRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreatedAt => serializer.serialize_str("created_at"),
            Self::UpdatedAt => serializer.serialize_str("updated_at"),
            Self::EndTime => serializer.serialize_str("end_time"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListFacebookAdsReportingRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "created_at" => Ok(Self::CreatedAt),
            "updated_at" => Ok(Self::UpdatedAt),
            "end_time" => Ok(Self::EndTime),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListFacebookAdsReportingRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreatedAt => write!(f, "created_at"),
            Self::UpdatedAt => write!(f, "updated_at"),
            Self::EndTime => write!(f, "end_time"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
