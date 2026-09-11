pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// High level audience information for who the outreach targets.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FacebookAdRecipients {
    /// The unique list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The status of the list used, namely if it's deleted or disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_is_active: Option<bool>,
    /// The name of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_name: Option<String>,
    /// Count of the recipients on the associated list. Formatted as an integer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_count: Option<i64>,
    /// An object representing all segmentation options. This object should contain a `saved_segment_id` to use an existing segment, or you can create a new segment by including both `match` and `conditions` options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_opts: Option<FacebookAdRecipientsSegmentOpts>,
    /// A description of the [segment](https://mailchimp.com/help/save-and-manage-segments/) used for the campaign. Formatted as a string marked up with HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_text: Option<String>,
}

impl FacebookAdRecipients {
    pub fn builder() -> FacebookAdRecipientsBuilder {
        <FacebookAdRecipientsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdRecipientsBuilder {
    list_id: Option<String>,
    list_is_active: Option<bool>,
    list_name: Option<String>,
    recipient_count: Option<i64>,
    segment_opts: Option<FacebookAdRecipientsSegmentOpts>,
    segment_text: Option<String>,
}

impl FacebookAdRecipientsBuilder {
    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn list_is_active(mut self, value: bool) -> Self {
        self.list_is_active = Some(value);
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

    pub fn segment_opts(mut self, value: FacebookAdRecipientsSegmentOpts) -> Self {
        self.segment_opts = Some(value);
        self
    }

    pub fn segment_text(mut self, value: impl Into<String>) -> Self {
        self.segment_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdRecipients`].
    pub fn build(self) -> Result<FacebookAdRecipients, BuildError> {
        Ok(FacebookAdRecipients {
            list_id: self.list_id,
            list_is_active: self.list_is_active,
            list_name: self.list_name,
            recipient_count: self.recipient_count,
            segment_opts: self.segment_opts,
            segment_text: self.segment_text,
        })
    }
}
