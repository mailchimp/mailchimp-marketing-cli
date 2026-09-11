pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignup {
        String(String),

        BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne(BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne),
}

impl BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignup {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_batch_subscribe_or_unsubscribe_lists_request_members_item_timestamp_signup_one(&self) -> bool {
        matches!(self, Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne(_))
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

    pub fn as_batch_subscribe_or_unsubscribe_lists_request_members_item_timestamp_signup_one(&self) -> Option<&BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne> {
        match self {
                    Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_batch_subscribe_or_unsubscribe_lists_request_members_item_timestamp_signup_one(self) -> Option<BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne> {
        match self {
                    Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampSignupOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
