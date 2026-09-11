pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An individual message in a conversation. Conversation tracking is a feature available to paid accounts that lets you view replies to your campaigns in your Mailchimp account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationMessage {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ConversationMessageLinksItem>>,
    /// A string that identifies this message's conversation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// A label representing the email of the sender of this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email: Option<String>,
    /// A label representing the sender of this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_label: Option<String>,
    /// A string that uniquely identifies this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The list's web ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i64>,
    /// The plain-text content of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Whether this message has been marked as read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// The subject of this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The date and time the message was either sent or received in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
}

impl ConversationMessage {
    pub fn builder() -> ConversationMessageBuilder {
        <ConversationMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationMessageBuilder {
    links: Option<Vec<ConversationMessageLinksItem>>,
    conversation_id: Option<String>,
    from_email: Option<String>,
    from_label: Option<String>,
    id: Option<String>,
    list_id: Option<i64>,
    message: Option<String>,
    read: Option<bool>,
    subject: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
}

impl ConversationMessageBuilder {
    pub fn links(mut self, value: Vec<ConversationMessageLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
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

    pub fn list_id(mut self, value: i64) -> Self {
        self.list_id = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn read(mut self, value: bool) -> Self {
        self.read = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationMessage`].
    pub fn build(self) -> Result<ConversationMessage, BuildError> {
        Ok(ConversationMessage {
            links: self.links,
            conversation_id: self.conversation_id,
            from_email: self.from_email,
            from_label: self.from_label,
            id: self.id,
            list_id: self.list_id,
            message: self.message,
            read: self.read,
            subject: self.subject,
            timestamp: self.timestamp,
        })
    }
}
