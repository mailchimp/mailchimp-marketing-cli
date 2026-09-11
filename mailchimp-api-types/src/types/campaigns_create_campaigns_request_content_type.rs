pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How the campaign's content is put together. The old drag and drop editor uses 'template' while the new editor uses 'multichannel'. Defaults to template.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateCampaignsRequestContentType {
    Template,
    Multichannel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateCampaignsRequestContentType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Template => serializer.serialize_str("template"),
            Self::Multichannel => serializer.serialize_str("multichannel"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateCampaignsRequestContentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "template" => Ok(Self::Template),
            "multichannel" => Ok(Self::Multichannel),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateCampaignsRequestContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Template => write!(f, "template"),
            Self::Multichannel => write!(f, "multichannel"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
