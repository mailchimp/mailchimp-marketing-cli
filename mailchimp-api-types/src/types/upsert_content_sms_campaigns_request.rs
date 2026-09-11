pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpsertContentSmsCampaignsRequest {
    /// The SMS message body.
    #[serde(default)]
    pub message_body: String,
    /// Attached images or files. Limited to one item. Omitting this field or sending an empty array removes any existing media; to keep the current media while updating other fields, re-send the media array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<UpsertContentSmsCampaignsRequestMediaItem>>,
}

impl UpsertContentSmsCampaignsRequest {
    pub fn builder() -> UpsertContentSmsCampaignsRequestBuilder {
        <UpsertContentSmsCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpsertContentSmsCampaignsRequestBuilder {
    message_body: Option<String>,
    media: Option<Vec<UpsertContentSmsCampaignsRequestMediaItem>>,
}

impl UpsertContentSmsCampaignsRequestBuilder {
    pub fn message_body(mut self, value: impl Into<String>) -> Self {
        self.message_body = Some(value.into());
        self
    }

    pub fn media(mut self, value: Vec<UpsertContentSmsCampaignsRequestMediaItem>) -> Self {
        self.media = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpsertContentSmsCampaignsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message_body`](UpsertContentSmsCampaignsRequestBuilder::message_body)
    pub fn build(self) -> Result<UpsertContentSmsCampaignsRequest, BuildError> {
        Ok(UpsertContentSmsCampaignsRequest {
            message_body: self.message_body.ok_or_else(|| BuildError::missing_field("message_body"))?,
            media: self.media,
        })
    }
}

