pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Determines how this category’s interests appear on signup forms.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateInterestCategoryListsRequestType {
    Checkboxes,
    Dropdown,
    Radio,
    Hidden,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateInterestCategoryListsRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Checkboxes => serializer.serialize_str("checkboxes"),
            Self::Dropdown => serializer.serialize_str("dropdown"),
            Self::Radio => serializer.serialize_str("radio"),
            Self::Hidden => serializer.serialize_str("hidden"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateInterestCategoryListsRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "checkboxes" => Ok(Self::Checkboxes),
            "dropdown" => Ok(Self::Dropdown),
            "radio" => Ok(Self::Radio),
            "hidden" => Ok(Self::Hidden),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateInterestCategoryListsRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Checkboxes => write!(f, "checkboxes"),
            Self::Dropdown => write!(f, "dropdown"),
            Self::Radio => write!(f, "radio"),
            Self::Hidden => write!(f, "hidden"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
