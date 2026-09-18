pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BatchCompletedAt {
        DateTime(
            #[serde(with = "crate::core::flexible_datetime::offset")]
            DateTime<FixedOffset>
        ),

        BatchCompletedAtOne(BatchCompletedAtOne),
}

impl BatchCompletedAt {
    pub fn is_date_time(&self) -> bool {
        matches!(self, Self::DateTime(_))
    }

    pub fn is_batch_completed_at_one(&self) -> bool {
        matches!(self, Self::BatchCompletedAtOne(_))
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

    pub fn as_batch_completed_at_one(&self) -> Option<&BatchCompletedAtOne> {
        match self {
                    Self::BatchCompletedAtOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_batch_completed_at_one(self) -> Option<BatchCompletedAtOne> {
        match self {
                    Self::BatchCompletedAtOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for BatchCompletedAt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DateTime(value) => write!(f, "{}", value),
            Self::BatchCompletedAtOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
