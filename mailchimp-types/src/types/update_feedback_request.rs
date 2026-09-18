pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateFeedbackRequest {
    /// The block id for the editable block that the feedback addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_id: Option<i64>,
    /// The status of feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_complete: Option<bool>,
    /// The content of the feedback.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl UpdateFeedbackRequest {
    pub fn builder() -> UpdateFeedbackRequestBuilder {
        <UpdateFeedbackRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeedbackRequestBuilder {
    block_id: Option<i64>,
    is_complete: Option<bool>,
    message: Option<String>,
}

impl UpdateFeedbackRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdateFeedbackRequest`].
    pub fn build(self) -> Result<UpdateFeedbackRequest, BuildError> {
        Ok(UpdateFeedbackRequest {
            block_id: self.block_id,
            is_complete: self.is_complete,
            message: self.message,
        })
    }
}

