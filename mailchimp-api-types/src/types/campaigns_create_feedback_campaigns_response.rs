pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A specific feedback message from a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFeedbackCampaignsResponse {
    /// The block id for the editable block that the feedback addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<i64>,
    /// The status of feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_complete: Option<bool>,
    /// The content of the feedback.
    #[serde(default)]
    pub message: String,
}

impl CreateFeedbackCampaignsResponse {
    pub fn builder() -> CreateFeedbackCampaignsResponseBuilder {
        <CreateFeedbackCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFeedbackCampaignsResponseBuilder {
    block_id: Option<i64>,
    is_complete: Option<bool>,
    message: Option<String>,
}

impl CreateFeedbackCampaignsResponseBuilder {
    pub fn block_id(mut self, value: i64) -> Self {
        self.block_id = Some(value);
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

    /// Consumes the builder and constructs a [`CreateFeedbackCampaignsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](CreateFeedbackCampaignsResponseBuilder::message)
    pub fn build(self) -> Result<CreateFeedbackCampaignsResponse, BuildError> {
        Ok(CreateFeedbackCampaignsResponse {
            block_id: self.block_id,
            is_complete: self.is_complete,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
