pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The day of the week to send a weekly RSS Campaign.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CampaignRssOptsScheduleWeeklySendDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CampaignRssOptsScheduleWeeklySendDay {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sunday => serializer.serialize_str("sunday"),
            Self::Monday => serializer.serialize_str("monday"),
            Self::Tuesday => serializer.serialize_str("tuesday"),
            Self::Wednesday => serializer.serialize_str("wednesday"),
            Self::Thursday => serializer.serialize_str("thursday"),
            Self::Friday => serializer.serialize_str("friday"),
            Self::Saturday => serializer.serialize_str("saturday"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CampaignRssOptsScheduleWeeklySendDay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sunday" => Ok(Self::Sunday),
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            "saturday" => Ok(Self::Saturday),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CampaignRssOptsScheduleWeeklySendDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sunday => write!(f, "sunday"),
            Self::Monday => write!(f, "monday"),
            Self::Tuesday => write!(f, "tuesday"),
            Self::Wednesday => write!(f, "wednesday"),
            Self::Thursday => write!(f, "thursday"),
            Self::Friday => write!(f, "friday"),
            Self::Saturday => write!(f, "saturday"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
