pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Details about an individual conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Conversation {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ConversationLinksItem>>,
    /// The unique identifier of the campaign for this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// A label representing the email of the sender of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email: Option<String>,
    /// A label representing the sender of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_label: Option<String>,
    /// A string that uniquely identifies this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The most recent message in the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message: Option<ConversationLastMessage>,
    /// The unique identifier of the list for this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of messages in this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i64>,
    /// The subject of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The number of unread messages in this conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread_messages: Option<i64>,
}

impl Conversation {
    pub fn builder() -> ConversationBuilder {
        <ConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationBuilder {
    links: Option<Vec<ConversationLinksItem>>,
    campaign_id: Option<String>,
    from_email: Option<String>,
    from_label: Option<String>,
    id: Option<String>,
    last_message: Option<ConversationLastMessage>,
    list_id: Option<String>,
    message_count: Option<i64>,
    subject: Option<String>,
    unread_messages: Option<i64>,
}

impl ConversationBuilder {
    pub fn links(mut self, value: Vec<ConversationLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn from_email(mut self, value: impl Into<String>) -> Self {
        self.from_email = Some(value.into());
        self
    }

    pub fn from_label(mut self, value: impl Into<String>) -> Self {
        self.from_label = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_message(mut self, value: ConversationLastMessage) -> Self {
        self.last_message = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn message_count(mut self, value: i64) -> Self {
        self.message_count = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn unread_messages(mut self, value: i64) -> Self {
        self.unread_messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Conversation`].
    pub fn build(self) -> Result<Conversation, BuildError> {
        Ok(Conversation {
            links: self.links,
            campaign_id: self.campaign_id,
            from_email: self.from_email,
            from_label: self.from_label,
            id: self.id,
            last_message: self.last_message,
            list_id: self.list_id,
            message_count: self.message_count,
            subject: self.subject,
            unread_messages: self.unread_messages,
        })
    }
}
