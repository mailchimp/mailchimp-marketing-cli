pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Messages from a specific conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMessagesConversationsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMessagesConversationsResponseLinksItem>>,
    /// A string that identifies this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// An array of objects, each representing a conversation messages resources.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_messages: Option<Vec<ConversationMessage>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMessagesConversationsResponse {
    pub fn builder() -> ListMessagesConversationsResponseBuilder {
        <ListMessagesConversationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesConversationsResponseBuilder {
    links: Option<Vec<ListMessagesConversationsResponseLinksItem>>,
    conversation_id: Option<String>,
    conversation_messages: Option<Vec<ConversationMessage>>,
    total_items: Option<i64>,
}

impl ListMessagesConversationsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMessagesConversationsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn conversation_messages(mut self, value: Vec<ConversationMessage>) -> Self {
        self.conversation_messages = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMessagesConversationsResponse`].
    pub fn build(self) -> Result<ListMessagesConversationsResponse, BuildError> {
        Ok(ListMessagesConversationsResponse {
            links: self.links,
            conversation_id: self.conversation_id,
            conversation_messages: self.conversation_messages,
            total_items: self.total_items,
        })
    }
}
