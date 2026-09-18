pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateRequestIncludeStagesItem {
    Audiences,
    Campaigns,
    Events,
    GalleryFiles,
    Reports,
    Templates,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateRequestIncludeStagesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Audiences => serializer.serialize_str("audiences"),
            Self::Campaigns => serializer.serialize_str("campaigns"),
            Self::Events => serializer.serialize_str("events"),
            Self::GalleryFiles => serializer.serialize_str("gallery_files"),
            Self::Reports => serializer.serialize_str("reports"),
            Self::Templates => serializer.serialize_str("templates"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateRequestIncludeStagesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "audiences" => Ok(Self::Audiences),
            "campaigns" => Ok(Self::Campaigns),
            "events" => Ok(Self::Events),
            "gallery_files" => Ok(Self::GalleryFiles),
            "reports" => Ok(Self::Reports),
            "templates" => Ok(Self::Templates),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateRequestIncludeStagesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Audiences => write!(f, "audiences"),
            Self::Campaigns => write!(f, "campaigns"),
            Self::Events => write!(f, "events"),
            Self::GalleryFiles => write!(f, "gallery_files"),
            Self::Reports => write!(f, "reports"),
            Self::Templates => write!(f, "templates"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
