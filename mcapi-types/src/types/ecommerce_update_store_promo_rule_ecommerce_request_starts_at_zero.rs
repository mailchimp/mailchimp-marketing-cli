pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateStorePromoRuleEcommerceRequestStartsAtZero {
        DateTime(
            #[serde(with = "crate::core::flexible_datetime::offset")]
            DateTime<FixedOffset>
        ),

        Date(NaiveDate),

        String(String),
}

impl UpdateStorePromoRuleEcommerceRequestStartsAtZero {
    pub fn is_date_time(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    pub fn is_date(&self) -> bool {
        matches!(self, Self::Date(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }


    pub fn as_date_time(&self) -> Option<&DateTime<FixedOffset>> {
        match self {
                    Self::DateTime(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_date_time(self) -> Option<DateTime<FixedOffset>> {
        match self {
                    Self::DateTime(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_date(&self) -> Option<&NaiveDate> {
        match self {
                    Self::Date(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_date(self) -> Option<NaiveDate> {
        match self {
                    Self::Date(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpdateStorePromoRuleEcommerceRequestStartsAtZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateTime(value) => write!(f, "{}", value),
            Self::Date(value) => write!(f, "{}", value),
            Self::String(value) => write!(f, "{}", value),
        }
    }
}
