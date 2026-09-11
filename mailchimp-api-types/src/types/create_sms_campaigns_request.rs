pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSmsCampaignsRequest {
    /// The name of the campaign.
    #[serde(default)]
    pub name: String,
    /// The numeric ID of the list to send the campaign to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i64>,
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

impl CreateSmsCampaignsRequest {
    pub fn builder() -> CreateSmsCampaignsRequestBuilder {
        <CreateSmsCampaignsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSmsCampaignsRequestBuilder {
    name: Option<String>,
    list_id: Option<i64>,
    folder_id: Option<String>,
    segments: Option<Vec<i64>>,
    excluded_segments: Option<Vec<i64>>,
}

impl CreateSmsCampaignsRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn list_id(mut self, value: i64) -> Self {
        self.list_id = Some(value);
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

    /// Consumes the builder and constructs a [`CreateSmsCampaignsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateSmsCampaignsRequestBuilder::name)
    pub fn build(self) -> Result<CreateSmsCampaignsRequest, BuildError> {
        Ok(CreateSmsCampaignsRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            list_id: self.list_id,
            folder_id: self.folder_id,
            segments: self.segments,
            excluded_segments: self.excluded_segments,
        })
    }
}

