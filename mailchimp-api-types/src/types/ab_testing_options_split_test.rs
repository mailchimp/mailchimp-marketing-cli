pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of AB split to run.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbTestingOptionsSplitTest {
    Subject,
    FromName,
    Schedule,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AbTestingOptionsSplitTest {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Subject => serializer.serialize_str("subject"),
            Self::FromName => serializer.serialize_str("from_name"),
            Self::Schedule => serializer.serialize_str("schedule"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AbTestingOptionsSplitTest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "subject" => Ok(Self::Subject),
            "from_name" => Ok(Self::FromName),
            "schedule" => Ok(Self::Schedule),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AbTestingOptionsSplitTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Subject => write!(f, "subject"),
            Self::FromName => write!(f, "from_name"),
            Self::Schedule => write!(f, "schedule"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
