pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List settings for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateRequestRecipients {
    /// The unique list id.
    #[serde(default)]
    pub list_id: String,
    /// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<CreateRequestRecipientsSegmentOpts>,
}

impl CreateRequestRecipients {
    pub fn builder() -> CreateRequestRecipientsBuilder {
        <CreateRequestRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestRecipientsBuilder {
    list_id: Option<String>,
    segment_opts: Option<CreateRequestRecipientsSegmentOpts>,
}

impl CreateRequestRecipientsBuilder {
    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn segment_opts(mut self, value: CreateRequestRecipientsSegmentOpts) -> Self {
        self.segment_opts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequestRecipients`].
    /// This method will fail if any of the following fields are not set:
    /// - [`list_id`](CreateRequestRecipientsBuilder::list_id)
    pub fn build(self) -> Result<CreateRequestRecipients, BuildError> {
        Ok(CreateRequestRecipients {
            list_id: self.list_id.ok_or_else(|| BuildError::missing_field("list_id"))?,
            segment_opts: self.segment_opts,
        })
    }
}
