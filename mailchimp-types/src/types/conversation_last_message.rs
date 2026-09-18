pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The most recent message in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationLastMessage {
    /// A label representing the email of the sender of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_email: Option<String>,
    /// A label representing the sender of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_label: Option<String>,
    /// The plain-text content of the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Whether this message has been marked as read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    /// The subject of this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The date and time the message was either sent or received in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
}

impl ConversationLastMessage {
    pub fn builder() -> ConversationLastMessageBuilder {
        <ConversationLastMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationLastMessageBuilder {
    from_email: Option<String>,
    from_label: Option<String>,
    message: Option<String>,
    read: Option<bool>,
    subject: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
}

impl ConversationLastMessageBuilder {
    pub fn from_email(mut self, value: impl Into<String>) -> Self {
        self.from_email = Some(value.into());
        self
    }

    pub fn from_label(mut self, value: impl Into<String>) -> Self {
        self.from_label = Some(value.into());
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

    /// Consumes the builder and constructs a [`ConversationLastMessage`].
    pub fn build(self) -> Result<ConversationLastMessage, BuildError> {
        Ok(ConversationLastMessage {
            from_email: self.from_email,
            from_label: self.from_label,
            message: self.message,
            read: self.read,
            subject: self.subject,
            timestamp: self.timestamp,
        })
    }
}
