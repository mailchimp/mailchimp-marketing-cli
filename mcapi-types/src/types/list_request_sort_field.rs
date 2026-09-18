pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListRequestSortField {
    DateCreated,
    DateEdited,
    Name,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DateCreated => serializer.serialize_str("date_created"),
            Self::DateEdited => serializer.serialize_str("date_edited"),
            Self::Name => serializer.serialize_str("name"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "date_created" => Ok(Self::DateCreated),
            "date_edited" => Ok(Self::DateEdited),
            "name" => Ok(Self::Name),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateCreated => write!(f, "date_created"),
            Self::DateEdited => write!(f, "date_edited"),
            Self::Name => write!(f, "name"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
