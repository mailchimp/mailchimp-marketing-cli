pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsListQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Whether the conversation has any unread messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_unread_messages: Option<ListConversationsRequestHasUnreadMessages>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
}

impl ConversationsListQueryRequest {
    pub fn builder() -> ConversationsListQueryRequestBuilder {
        <ConversationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    has_unread_messages: Option<ListConversationsRequestHasUnreadMessages>,
    list_id: Option<String>,
    campaign_id: Option<String>,
}

impl ConversationsListQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn has_unread_messages(mut self, value: ListConversationsRequestHasUnreadMessages) -> Self {
        self.has_unread_messages = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ConversationsListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ConversationsListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ConversationsListQueryRequest, BuildError> {
        Ok(ConversationsListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            has_unread_messages: self.has_unread_messages,
            list_id: self.list_id,
            campaign_id: self.campaign_id,
        })
    }
}

