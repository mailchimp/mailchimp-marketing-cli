pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateMemberListsRequestTimestampSignup {
        String(String),

        CreateMemberListsRequestTimestampSignupOne(CreateMemberListsRequestTimestampSignupOne),
}

impl CreateMemberListsRequestTimestampSignup {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_create_member_lists_request_timestamp_signup_one(&self) -> bool {
        matches!(self, Self::CreateMemberListsRequestTimestampSignupOne(_))
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

    pub fn as_create_member_lists_request_timestamp_signup_one(&self) -> Option<&CreateMemberListsRequestTimestampSignupOne> {
        match self {
                    Self::CreateMemberListsRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_member_lists_request_timestamp_signup_one(self) -> Option<CreateMemberListsRequestTimestampSignupOne> {
        match self {
                    Self::CreateMemberListsRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateMemberListsRequestTimestampSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::CreateMemberListsRequestTimestampSignupOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
