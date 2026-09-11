pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of this account's tracked conversations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListConversationsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListConversationsResponseLinksItem>>,
    /// A list of conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversations: Option<Vec<Conversation>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListConversationsResponse {
    pub fn builder() -> ListConversationsResponseBuilder {
        <ListConversationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListConversationsResponseBuilder {
    links: Option<Vec<ListConversationsResponseLinksItem>>,
    conversations: Option<Vec<Conversation>>,
    total_items: Option<i64>,
}

impl ListConversationsResponseBuilder {
    pub fn links(mut self, value: Vec<ListConversationsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn conversations(mut self, value: Vec<Conversation>) -> Self {
        self.conversations = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListConversationsResponse`].
    pub fn build(self) -> Result<ListConversationsResponse, BuildError> {
        Ok(ListConversationsResponse {
            links: self.links,
            conversations: self.conversations,
            total_items: self.total_items,
        })
    }
}
