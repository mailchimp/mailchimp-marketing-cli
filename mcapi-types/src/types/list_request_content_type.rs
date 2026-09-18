pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListRequestContentType {
    Html,
    Template,
    Multichannel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListRequestContentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Html => serializer.serialize_str("html"),
            Self::Template => serializer.serialize_str("template"),
            Self::Multichannel => serializer.serialize_str("multichannel"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListRequestContentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "html" => Ok(Self::Html),
            "template" => Ok(Self::Template),
            "multichannel" => Ok(Self::Multichannel),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListRequestContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Html => write!(f, "html"),
            Self::Template => write!(f, "template"),
            Self::Multichannel => write!(f, "multichannel"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
