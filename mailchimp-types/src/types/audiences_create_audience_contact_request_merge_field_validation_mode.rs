pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAudienceContactRequestMergeFieldValidationMode {
    IgnoreRequiredChecks,
    Strict,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAudienceContactRequestMergeFieldValidationMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::IgnoreRequiredChecks => serializer.serialize_str("ignore_required_checks"),
            Self::Strict => serializer.serialize_str("strict"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAudienceContactRequestMergeFieldValidationMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ignore_required_checks" => Ok(Self::IgnoreRequiredChecks),
            "strict" => Ok(Self::Strict),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAudienceContactRequestMergeFieldValidationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IgnoreRequiredChecks => write!(f, "ignore_required_checks"),
            Self::Strict => write!(f, "strict"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
