pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListResponseFirstPayment {
        DateTime(
            #[serde(with = "crate::core::flexible_datetime::offset")]
            DateTime<FixedOffset>
        ),

        ListResponseFirstPaymentOne(ListResponseFirstPaymentOne),
}

impl ListResponseFirstPayment {
    pub fn is_date_time(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    pub fn is_list_response_first_payment_one(&self) -> bool {
        matches!(self, Self::ListResponseFirstPaymentOne(_))
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

    pub fn as_list_response_first_payment_one(&self) -> Option<&ListResponseFirstPaymentOne> {
        match self {
                    Self::ListResponseFirstPaymentOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_list_response_first_payment_one(self) -> Option<ListResponseFirstPaymentOne> {
        match self {
                    Self::ListResponseFirstPaymentOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ListResponseFirstPayment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateTime(value) => write!(f, "{}", value),
            Self::ListResponseFirstPaymentOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
