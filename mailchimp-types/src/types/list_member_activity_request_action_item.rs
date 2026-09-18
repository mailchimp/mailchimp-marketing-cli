pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListMemberActivityRequestActionItem {
    Abuse,
    Bounce,
    Click,
    Open,
    Sent,
    Unsub,
    Ecomm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListMemberActivityRequestActionItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Abuse => serializer.serialize_str("abuse"),
            Self::Bounce => serializer.serialize_str("bounce"),
            Self::Click => serializer.serialize_str("click"),
            Self::Open => serializer.serialize_str("open"),
            Self::Sent => serializer.serialize_str("sent"),
            Self::Unsub => serializer.serialize_str("unsub"),
            Self::Ecomm => serializer.serialize_str("ecomm"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListMemberActivityRequestActionItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "abuse" => Ok(Self::Abuse),
            "bounce" => Ok(Self::Bounce),
            "click" => Ok(Self::Click),
            "open" => Ok(Self::Open),
            "sent" => Ok(Self::Sent),
            "unsub" => Ok(Self::Unsub),
            "ecomm" => Ok(Self::Ecomm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListMemberActivityRequestActionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Abuse => write!(f, "abuse"),
            Self::Bounce => write!(f, "bounce"),
            Self::Click => write!(f, "click"),
            Self::Open => write!(f, "open"),
            Self::Sent => write!(f, "sent"),
            Self::Unsub => write!(f, "unsub"),
            Self::Ecomm => write!(f, "ecomm"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
