pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// List settings for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CampaignRecipients {
    /// The unique list id.
    #[serde(default)]
    pub list_id: String,
    /// The name of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// Count of the recipients on the associated list. Formatted as an integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_count: Option<i64>,
    /// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<CampaignRecipientsSegmentOpts>,
    /// A description of the [segment](https://mailchimp.com/help/save-and-manage-segments/) used for the campaign. Formatted as a string marked up with HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_text: Option<String>,
}

impl CampaignRecipients {
    pub fn builder() -> CampaignRecipientsBuilder {
        <CampaignRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignRecipientsBuilder {
    list_id: Option<String>,
    list_name: Option<String>,
    recipient_count: Option<i64>,
    segment_opts: Option<CampaignRecipientsSegmentOpts>,
    segment_text: Option<String>,
}

impl CampaignRecipientsBuilder {
    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn list_name(mut self, value: impl Into<String>) -> Self {
        self.list_name = Some(value.into());
        self
    }

    pub fn recipient_count(mut self, value: i64) -> Self {
        self.recipient_count = Some(value);
        self
    }

    pub fn segment_opts(mut self, value: CampaignRecipientsSegmentOpts) -> Self {
        self.segment_opts = Some(value);
        self
    }

    pub fn segment_text(mut self, value: impl Into<String>) -> Self {
        self.segment_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CampaignRecipients`].
    /// This method will fail if any of the following fields are not set:
    /// - [`list_id`](CampaignRecipientsBuilder::list_id)
    pub fn build(self) -> Result<CampaignRecipients, BuildError> {
        Ok(CampaignRecipients {
            list_id: self.list_id.ok_or_else(|| BuildError::missing_field("list_id"))?,
            list_name: self.list_name,
            recipient_count: self.recipient_count,
            segment_opts: self.segment_opts,
            segment_text: self.segment_text,
        })
    }
}
