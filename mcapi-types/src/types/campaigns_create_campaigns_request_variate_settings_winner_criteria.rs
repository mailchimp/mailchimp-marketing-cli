pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The combination that performs the best. This may be determined automatically by click rate, open rate, or total revenue -- or you may choose manually based on the reporting data you find the most valuable. For Multivariate Campaigns testing send_time, winner_criteria is ignored. For Multivariate Campaigns with 'manual' as the winner_criteria, the winner must be chosen in the Mailchimp web application.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateCampaignsRequestVariateSettingsWinnerCriteria {
    Opens,
    Clicks,
    Manual,
    TotalRevenue,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateCampaignsRequestVariateSettingsWinnerCriteria {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Opens => serializer.serialize_str("opens"),
            Self::Clicks => serializer.serialize_str("clicks"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::TotalRevenue => serializer.serialize_str("total_revenue"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateCampaignsRequestVariateSettingsWinnerCriteria {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "opens" => Ok(Self::Opens),
            "clicks" => Ok(Self::Clicks),
            "manual" => Ok(Self::Manual),
            "total_revenue" => Ok(Self::TotalRevenue),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateCampaignsRequestVariateSettingsWinnerCriteria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Opens => write!(f, "opens"),
            Self::Clicks => write!(f, "clicks"),
            Self::Manual => write!(f, "manual"),
            Self::TotalRevenue => write!(f, "total_revenue"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
