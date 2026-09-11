pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOpt {
        String(String),

        BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne(BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne),
}

impl BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOpt {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_batch_subscribe_or_unsubscribe_lists_request_members_item_timestamp_opt_one(&self) -> bool {
        matches!(self, Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne(_))
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

    pub fn as_batch_subscribe_or_unsubscribe_lists_request_members_item_timestamp_opt_one(&self) -> Option<&BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne> {
        match self {
                    Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_batch_subscribe_or_unsubscribe_lists_request_members_item_timestamp_opt_one(self) -> Option<BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne> {
        match self {
                    Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOpt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::BatchSubscribeOrUnsubscribeListsRequestMembersItemTimestampOptOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
