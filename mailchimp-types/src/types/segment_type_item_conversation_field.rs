pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Segment by interaction with a campaign via Conversations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SegmentTypeItemConversationField {
    #[serde(rename = "conversation")]
    Conversation,
}
impl fmt::Display for SegmentTypeItemConversationField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Conversation => "conversation",
        };
        write!(f, "{}", s)
    }
}
