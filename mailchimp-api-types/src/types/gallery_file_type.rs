pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of file in the File Manager.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GalleryFileType {
    Image,
    File,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GalleryFileType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Image => serializer.serialize_str("image"),
            Self::File => serializer.serialize_str("file"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GalleryFileType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "image" => Ok(Self::Image),
            "file" => Ok(Self::File),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GalleryFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Image => write!(f, "image"),
            Self::File => write!(f, "file"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
