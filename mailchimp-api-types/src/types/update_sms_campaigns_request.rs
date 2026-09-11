pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSmsCampaignsRequest {
    /// The name of the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The ID of the folder to place this campaign in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// The segment IDs to target for this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<i64>>,
    /// The segment IDs to exclude from this campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_segments: Option<Vec<i64>>,
}

impl UpdateSmsCampaignsRequest {
    pub fn builder() -> UpdateSmsCampaignsRequestBuilder {
        <UpdateSmsCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSmsCampaignsRequestBuilder {
    name: Option<String>,
    folder_id: Option<String>,
    segments: Option<Vec<i64>>,
    excluded_segments: Option<Vec<i64>>,
}

impl UpdateSmsCampaignsRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn segments(mut self, value: Vec<i64>) -> Self {
        self.segments = Some(value);
        self
    }

    pub fn excluded_segments(mut self, value: Vec<i64>) -> Self {
        self.excluded_segments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateSmsCampaignsRequest`].
    pub fn build(self) -> Result<UpdateSmsCampaignsRequest, BuildError> {
        Ok(UpdateSmsCampaignsRequest {
            name: self.name,
            folder_id: self.folder_id,
            segments: self.segments,
            excluded_segments: self.excluded_segments,
        })
    }
}

