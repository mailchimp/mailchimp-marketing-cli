pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How the template's content is put together.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TemplateInstanceContentType {
    Template,
    Multichannel,
    Html,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TemplateInstanceContentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Template => serializer.serialize_str("template"),
            Self::Multichannel => serializer.serialize_str("multichannel"),
            Self::Html => serializer.serialize_str("html"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TemplateInstanceContentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "template" => Ok(Self::Template),
            "multichannel" => Ok(Self::Multichannel),
            "html" => Ok(Self::Html),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TemplateInstanceContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template => write!(f, "template"),
            Self::Multichannel => write!(f, "multichannel"),
            Self::Html => write!(f, "html"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
