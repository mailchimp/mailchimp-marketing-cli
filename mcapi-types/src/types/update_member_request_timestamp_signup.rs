pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum UpdateMemberRequestTimestampSignup {
        String(String),

        UpdateMemberRequestTimestampSignupOne(UpdateMemberRequestTimestampSignupOne),
}

impl UpdateMemberRequestTimestampSignup {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_update_member_request_timestamp_signup_one(&self) -> bool {
        matches!(self, Self::UpdateMemberRequestTimestampSignupOne(_))
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

    pub fn as_update_member_request_timestamp_signup_one(&self) -> Option<&UpdateMemberRequestTimestampSignupOne> {
        match self {
                    Self::UpdateMemberRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_update_member_request_timestamp_signup_one(self) -> Option<UpdateMemberRequestTimestampSignupOne> {
        match self {
                    Self::UpdateMemberRequestTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for UpdateMemberRequestTimestampSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::UpdateMemberRequestTimestampSignupOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
