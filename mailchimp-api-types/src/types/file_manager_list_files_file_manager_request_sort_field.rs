pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListFilesFileManagerRequestSortField {
    AddedDate,
    Name,
    Size,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListFilesFileManagerRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AddedDate => serializer.serialize_str("added_date"),
            Self::Name => serializer.serialize_str("name"),
            Self::Size => serializer.serialize_str("size"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListFilesFileManagerRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "added_date" => Ok(Self::AddedDate),
            "name" => Ok(Self::Name),
            "size" => Ok(Self::Size),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListFilesFileManagerRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddedDate => write!(f, "added_date"),
            Self::Name => write!(f, "name"),
            Self::Size => write!(f, "size"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
