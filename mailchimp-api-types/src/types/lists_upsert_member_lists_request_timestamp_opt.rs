pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpsertMemberListsRequestTimestampOpt {
        String(String),

        UpsertMemberListsRequestTimestampOptOne(UpsertMemberListsRequestTimestampOptOne),
}

impl UpsertMemberListsRequestTimestampOpt {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_upsert_member_lists_request_timestamp_opt_one(&self) -> bool {
        matches!(self, Self::UpsertMemberListsRequestTimestampOptOne(_))
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

    pub fn as_upsert_member_lists_request_timestamp_opt_one(&self) -> Option<&UpsertMemberListsRequestTimestampOptOne> {
        match self {
                    Self::UpsertMemberListsRequestTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_upsert_member_lists_request_timestamp_opt_one(self) -> Option<UpsertMemberListsRequestTimestampOptOne> {
        match self {
                    Self::UpsertMemberListsRequestTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpsertMemberListsRequestTimestampOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::UpsertMemberListsRequestTimestampOptOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
