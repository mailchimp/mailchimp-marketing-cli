pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List settings for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateCampaignsRequestRecipients {
    /// The unique list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<UpdateCampaignsRequestRecipientsSegmentOpts>,
}

impl UpdateCampaignsRequestRecipients {
    pub fn builder() -> UpdateCampaignsRequestRecipientsBuilder {
        <UpdateCampaignsRequestRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCampaignsRequestRecipientsBuilder {
    list_id: Option<String>,
    segment_opts: Option<UpdateCampaignsRequestRecipientsSegmentOpts>,
}

impl UpdateCampaignsRequestRecipientsBuilder {
    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn segment_opts(mut self, value: UpdateCampaignsRequestRecipientsSegmentOpts) -> Self {
        self.segment_opts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCampaignsRequestRecipients`].
    pub fn build(self) -> Result<UpdateCampaignsRequestRecipients, BuildError> {
        Ok(UpdateCampaignsRequestRecipients {
            list_id: self.list_id,
            segment_opts: self.segment_opts,
        })
    }
}
