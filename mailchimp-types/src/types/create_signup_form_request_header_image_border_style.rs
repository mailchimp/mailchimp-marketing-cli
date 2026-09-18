pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Image border style.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateSignupFormRequestHeaderImageBorderStyle {
    None,
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Outset,
    Inset,
    Ridge,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateSignupFormRequestHeaderImageBorderStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("none"),
            Self::Solid => serializer.serialize_str("solid"),
            Self::Dotted => serializer.serialize_str("dotted"),
            Self::Dashed => serializer.serialize_str("dashed"),
            Self::Double => serializer.serialize_str("double"),
            Self::Groove => serializer.serialize_str("groove"),
            Self::Outset => serializer.serialize_str("outset"),
            Self::Inset => serializer.serialize_str("inset"),
            Self::Ridge => serializer.serialize_str("ridge"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateSignupFormRequestHeaderImageBorderStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "none" => Ok(Self::None),
            "solid" => Ok(Self::Solid),
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "double" => Ok(Self::Double),
            "groove" => Ok(Self::Groove),
            "outset" => Ok(Self::Outset),
            "inset" => Ok(Self::Inset),
            "ridge" => Ok(Self::Ridge),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateSignupFormRequestHeaderImageBorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Solid => write!(f, "solid"),
            Self::Dotted => write!(f, "dotted"),
            Self::Dashed => write!(f, "dashed"),
            Self::Double => write!(f, "double"),
            Self::Groove => write!(f, "groove"),
            Self::Outset => write!(f, "outset"),
            Self::Inset => write!(f, "inset"),
            Self::Ridge => write!(f, "ridge"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
