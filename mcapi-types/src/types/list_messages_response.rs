pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Messages from a specific conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMessagesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMessagesResponseLinksItem>>,
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

impl ListMessagesResponse {
    pub fn builder() -> ListMessagesResponseBuilder {
        <ListMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesResponseBuilder {
    links: Option<Vec<ListMessagesResponseLinksItem>>,
    conversation_id: Option<String>,
    conversation_messages: Option<Vec<ConversationMessage>>,
    total_items: Option<i64>,
}

impl ListMessagesResponseBuilder {
    pub fn links(mut self, value: Vec<ListMessagesResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListMessagesResponse`].
    pub fn build(self) -> Result<ListMessagesResponse, BuildError> {
        Ok(ListMessagesResponse {
            links: self.links,
            conversation_id: self.conversation_id,
            conversation_messages: self.conversation_messages,
            total_items: self.total_items,
        })
    }
}
