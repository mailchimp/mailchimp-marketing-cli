pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpsertMemberRequestTimestampSignup {
        String(String),

        UpsertMemberRequestTimestampSignupOne(UpsertMemberRequestTimestampSignupOne),
}

impl UpsertMemberRequestTimestampSignup {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_upsert_member_request_timestamp_signup_one(&self) -> bool {
        matches!(self, Self::UpsertMemberRequestTimestampSignupOne(_))
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

    pub fn as_upsert_member_request_timestamp_signup_one(&self) -> Option<&UpsertMemberRequestTimestampSignupOne> {
        match self {
                    Self::UpsertMemberRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_upsert_member_request_timestamp_signup_one(self) -> Option<UpsertMemberRequestTimestampSignupOne> {
        match self {
                    Self::UpsertMemberRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpsertMemberRequestTimestampSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::UpsertMemberRequestTimestampSignupOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
