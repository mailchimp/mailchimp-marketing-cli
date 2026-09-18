pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateMemberRequestTimestampOpt {
        String(String),

        CreateMemberRequestTimestampOptOne(CreateMemberRequestTimestampOptOne),
}

impl CreateMemberRequestTimestampOpt {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_create_member_request_timestamp_opt_one(&self) -> bool {
        matches!(self, Self::CreateMemberRequestTimestampOptOne(_))
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

    pub fn as_create_member_request_timestamp_opt_one(&self) -> Option<&CreateMemberRequestTimestampOptOne> {
        match self {
                    Self::CreateMemberRequestTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_member_request_timestamp_opt_one(self) -> Option<CreateMemberRequestTimestampOptOne> {
        match self {
                    Self::CreateMemberRequestTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateMemberRequestTimestampOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::CreateMemberRequestTimestampOptOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
