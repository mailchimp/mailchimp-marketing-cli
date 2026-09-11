pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-messages
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMessagesQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// Whether a conversation message has been marked as read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read: Option<ListMessagesConversationsRequestIsRead>,
    /// Restrict the response to messages created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub before_timestamp: Option<DateTime<FixedOffset>>,
    /// Restrict the response to messages created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub since_timestamp: Option<DateTime<FixedOffset>>,
}

impl ListMessagesQueryRequest {
    pub fn builder() -> ListMessagesQueryRequestBuilder {
        <ListMessagesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMessagesQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    is_read: Option<ListMessagesConversationsRequestIsRead>,
    before_timestamp: Option<DateTime<FixedOffset>>,
    since_timestamp: Option<DateTime<FixedOffset>>,
}

impl ListMessagesQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn is_read(mut self, value: ListMessagesConversationsRequestIsRead) -> Self {
        self.is_read = Some(value);
        self
    }

    pub fn before_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.before_timestamp = Some(value);
        self
    }

    pub fn since_timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMessagesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListMessagesQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListMessagesQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListMessagesQueryRequest, BuildError> {
        Ok(ListMessagesQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            is_read: self.is_read,
            before_timestamp: self.before_timestamp,
            since_timestamp: self.since_timestamp,
        })
    }
}

