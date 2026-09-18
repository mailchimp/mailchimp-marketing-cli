pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A specific feedback message from a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFeedbackResponseFeedbackItem {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFeedbackResponseFeedbackItemLinksItem>>,
    /// The block id for the editable block that the feedback addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<i64>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// The date and time the feedback item was created in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The login name of the user who created the feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// The individual id for the feedback item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback_id: Option<i64>,
    /// The status of feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_complete: Option<bool>,
    /// The content of the feedback.
    #[serde(default)]
    pub message: String,
    /// If a reply, the id of the parent feedback item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    /// The source of the feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ListFeedbackResponseFeedbackItemSource>,
    /// The date and time the feedback was last updated in ISO 8601 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_at: Option<DateTime<FixedOffset>>,
}

impl ListFeedbackResponseFeedbackItem {
    pub fn builder() -> ListFeedbackResponseFeedbackItemBuilder {
        <ListFeedbackResponseFeedbackItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFeedbackResponseFeedbackItemBuilder {
    links: Option<Vec<ListFeedbackResponseFeedbackItemLinksItem>>,
    block_id: Option<i64>,
    campaign_id: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    created_by: Option<String>,
    feedback_id: Option<i64>,
    is_complete: Option<bool>,
    message: Option<String>,
    parent_id: Option<i64>,
    source: Option<ListFeedbackResponseFeedbackItemSource>,
    updated_at: Option<DateTime<FixedOffset>>,
}

impl ListFeedbackResponseFeedbackItemBuilder {
    pub fn links(mut self, value: Vec<ListFeedbackResponseFeedbackItemLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn block_id(mut self, value: i64) -> Self {
        self.block_id = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn feedback_id(mut self, value: i64) -> Self {
        self.feedback_id = Some(value);
        self
    }

    pub fn is_complete(mut self, value: bool) -> Self {
        self.is_complete = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn parent_id(mut self, value: i64) -> Self {
        self.parent_id = Some(value);
        self
    }

    pub fn source(mut self, value: ListFeedbackResponseFeedbackItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFeedbackResponseFeedbackItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ListFeedbackResponseFeedbackItemBuilder::message)
    pub fn build(self) -> Result<ListFeedbackResponseFeedbackItem, BuildError> {
        Ok(ListFeedbackResponseFeedbackItem {
            links: self.links,
            block_id: self.block_id,
            campaign_id: self.campaign_id,
            created_at: self.created_at,
            created_by: self.created_by,
            feedback_id: self.feedback_id,
            is_complete: self.is_complete,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            parent_id: self.parent_id,
            source: self.source,
            updated_at: self.updated_at,
        })
    }
}
