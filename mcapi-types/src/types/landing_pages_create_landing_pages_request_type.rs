pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of template the landing page has.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateLandingPagesRequestType {
    Signup,
    Product,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateLandingPagesRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Signup => serializer.serialize_str("signup"),
            Self::Product => serializer.serialize_str("product"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateLandingPagesRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "signup" => Ok(Self::Signup),
            "product" => Ok(Self::Product),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateLandingPagesRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Signup => write!(f, "signup"),
            Self::Product => write!(f, "product"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
