pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The [type](https://mailchimp.com/developer/marketing/docs/merge-fields/#structure) for the merge field.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateMergeFieldListsRequestType {
    Text,
    Number,
    Address,
    Phone,
    Date,
    Url,
    Imageurl,
    Radio,
    Dropdown,
    Birthday,
    Zip,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateMergeFieldListsRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::Number => serializer.serialize_str("number"),
            Self::Address => serializer.serialize_str("address"),
            Self::Phone => serializer.serialize_str("phone"),
            Self::Date => serializer.serialize_str("date"),
            Self::Url => serializer.serialize_str("url"),
            Self::Imageurl => serializer.serialize_str("imageurl"),
            Self::Radio => serializer.serialize_str("radio"),
            Self::Dropdown => serializer.serialize_str("dropdown"),
            Self::Birthday => serializer.serialize_str("birthday"),
            Self::Zip => serializer.serialize_str("zip"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateMergeFieldListsRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "number" => Ok(Self::Number),
            "address" => Ok(Self::Address),
            "phone" => Ok(Self::Phone),
            "date" => Ok(Self::Date),
            "url" => Ok(Self::Url),
            "imageurl" => Ok(Self::Imageurl),
            "radio" => Ok(Self::Radio),
            "dropdown" => Ok(Self::Dropdown),
            "birthday" => Ok(Self::Birthday),
            "zip" => Ok(Self::Zip),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateMergeFieldListsRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Number => write!(f, "number"),
            Self::Address => write!(f, "address"),
            Self::Phone => write!(f, "phone"),
            Self::Date => write!(f, "date"),
            Self::Url => write!(f, "url"),
            Self::Imageurl => write!(f, "imageurl"),
            Self::Radio => write!(f, "radio"),
            Self::Dropdown => write!(f, "dropdown"),
            Self::Birthday => write!(f, "birthday"),
            Self::Zip => write!(f, "zip"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
