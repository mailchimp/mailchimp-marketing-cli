pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Image alignment.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SignupFormHeaderImageAlign {
    None,
    Left,
    Center,
    Right,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SignupFormHeaderImageAlign {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Left => serializer.serialize_str("left"),
            Self::Center => serializer.serialize_str("center"),
            Self::Right => serializer.serialize_str("right"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SignupFormHeaderImageAlign {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SignupFormHeaderImageAlign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Left => write!(f, "left"),
            Self::Center => write!(f, "center"),
            Self::Right => write!(f, "right"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
