pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The status for the tag on the member, pass in active to add a tag or inactive to remove it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateMemberTagListsRequestTagsItemStatus {
    Inactive,
    Active,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateMemberTagListsRequestTagsItemStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Inactive => serializer.serialize_str("inactive"),
            Self::Active => serializer.serialize_str("active"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateMemberTagListsRequestTagsItemStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "inactive" => Ok(Self::Inactive),
            "active" => Ok(Self::Active),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateMemberTagListsRequestTagsItemStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Inactive => write!(f, "inactive"),
            Self::Active => write!(f, "active"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
