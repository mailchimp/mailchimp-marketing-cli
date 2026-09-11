pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateMemberListsRequestTimestampOpt {
        String(String),

        UpdateMemberListsRequestTimestampOptOne(UpdateMemberListsRequestTimestampOptOne),
}

impl UpdateMemberListsRequestTimestampOpt {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_update_member_lists_request_timestamp_opt_one(&self) -> bool {
        matches!(self, Self::UpdateMemberListsRequestTimestampOptOne(_))
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

    pub fn as_update_member_lists_request_timestamp_opt_one(&self) -> Option<&UpdateMemberListsRequestTimestampOptOne> {
        match self {
                    Self::UpdateMemberListsRequestTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_member_lists_request_timestamp_opt_one(self) -> Option<UpdateMemberListsRequestTimestampOptOne> {
        match self {
                    Self::UpdateMemberListsRequestTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpdateMemberListsRequestTimestampOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::UpdateMemberListsRequestTimestampOptOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
