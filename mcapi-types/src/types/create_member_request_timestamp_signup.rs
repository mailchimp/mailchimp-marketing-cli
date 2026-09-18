pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateMemberRequestTimestampSignup {
        String(String),

        CreateMemberRequestTimestampSignupOne(CreateMemberRequestTimestampSignupOne),
}

impl CreateMemberRequestTimestampSignup {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_create_member_request_timestamp_signup_one(&self) -> bool {
        matches!(self, Self::CreateMemberRequestTimestampSignupOne(_))
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

    pub fn as_create_member_request_timestamp_signup_one(&self) -> Option<&CreateMemberRequestTimestampSignupOne> {
        match self {
                    Self::CreateMemberRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_member_request_timestamp_signup_one(self) -> Option<CreateMemberRequestTimestampSignupOne> {
        match self {
                    Self::CreateMemberRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateMemberRequestTimestampSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::CreateMemberRequestTimestampSignupOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
