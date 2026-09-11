pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMembersListsRequestInterestMatch {
    Any,
    All,
    None,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMembersListsRequestInterestMatch {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Any => serializer.serialize_str("any"),
            Self::All => serializer.serialize_str("all"),
            Self::None => serializer.serialize_str("none"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMembersListsRequestInterestMatch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "any" => Ok(Self::Any),
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMembersListsRequestInterestMatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::All => write!(f, "all"),
            Self::None => write!(f, "none"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
