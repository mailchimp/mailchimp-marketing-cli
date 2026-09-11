pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of segment. Static segments are now known as tags. Learn more about [tags](https://mailchimp.com/help/getting-started-tags?utm_source=mc-api&utm_medium=docs&utm_campaign=apidocs).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListType {
    Saved,
    Static,
    Fuzzy,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Saved => serializer.serialize_str("saved"),
            Self::Static => serializer.serialize_str("static"),
            Self::Fuzzy => serializer.serialize_str("fuzzy"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "saved" => Ok(Self::Saved),
            "static" => Ok(Self::Static),
            "fuzzy" => Ok(Self::Fuzzy),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Saved => write!(f, "saved"),
            Self::Static => write!(f, "static"),
            Self::Fuzzy => write!(f, "fuzzy"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
