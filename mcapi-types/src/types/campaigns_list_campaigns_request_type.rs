pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListCampaignsRequestType {
    Regular,
    Plaintext,
    Absplit,
    Rss,
    Variate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListCampaignsRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Regular => serializer.serialize_str("regular"),
            Self::Plaintext => serializer.serialize_str("plaintext"),
            Self::Absplit => serializer.serialize_str("absplit"),
            Self::Rss => serializer.serialize_str("rss"),
            Self::Variate => serializer.serialize_str("variate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListCampaignsRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "regular" => Ok(Self::Regular),
            "plaintext" => Ok(Self::Plaintext),
            "absplit" => Ok(Self::Absplit),
            "rss" => Ok(Self::Rss),
            "variate" => Ok(Self::Variate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListCampaignsRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "regular"),
            Self::Plaintext => write!(f, "plaintext"),
            Self::Absplit => write!(f, "absplit"),
            Self::Rss => write!(f, "rss"),
            Self::Variate => write!(f, "variate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
