pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ListRootResponseFirstPayment {
        DateTime(
            #[serde(with = "crate::core::flexible_datetime::offset")]
            DateTime<FixedOffset>
        ),

        ListRootResponseFirstPaymentOne(ListRootResponseFirstPaymentOne),
}

impl ListRootResponseFirstPayment {
    pub fn is_date_time(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    pub fn is_list_root_response_first_payment_one(&self) -> bool {
        matches!(self, Self::ListRootResponseFirstPaymentOne(_))
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

    pub fn as_list_root_response_first_payment_one(&self) -> Option<&ListRootResponseFirstPaymentOne> {
        match self {
                    Self::ListRootResponseFirstPaymentOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_list_root_response_first_payment_one(self) -> Option<ListRootResponseFirstPaymentOne> {
        match self {
                    Self::ListRootResponseFirstPaymentOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ListRootResponseFirstPayment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateTime(value) => write!(f, "{}", value),
            Self::ListRootResponseFirstPaymentOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
