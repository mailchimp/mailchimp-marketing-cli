pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListInterestCategoriesListsRequestSortField {
    Name,
    DisplayOrder,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListInterestCategoriesListsRequestSortField {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Name => serializer.serialize_str("name"),
            Self::DisplayOrder => serializer.serialize_str("display_order"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListInterestCategoriesListsRequestSortField {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "name" => Ok(Self::Name),
            "display_order" => Ok(Self::DisplayOrder),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListInterestCategoriesListsRequestSortField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name => write!(f, "name"),
            Self::DisplayOrder => write!(f, "display_order"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
