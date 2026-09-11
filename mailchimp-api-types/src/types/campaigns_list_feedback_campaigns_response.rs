pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of the comment feedback for a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFeedbackCampaignsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFeedbackCampaignsResponseLinksItem>>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// A collection of feedback items for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Vec<ListFeedbackCampaignsResponseFeedbackItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFeedbackCampaignsResponse {
    pub fn builder() -> ListFeedbackCampaignsResponseBuilder {
        <ListFeedbackCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFeedbackCampaignsResponseBuilder {
    links: Option<Vec<ListFeedbackCampaignsResponseLinksItem>>,
    campaign_id: Option<String>,
    feedback: Option<Vec<ListFeedbackCampaignsResponseFeedbackItem>>,
    total_items: Option<i64>,
}

impl ListFeedbackCampaignsResponseBuilder {
    pub fn links(mut self, value: Vec<ListFeedbackCampaignsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn feedback(mut self, value: Vec<ListFeedbackCampaignsResponseFeedbackItem>) -> Self {
        self.feedback = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFeedbackCampaignsResponse`].
    pub fn build(self) -> Result<ListFeedbackCampaignsResponse, BuildError> {
        Ok(ListFeedbackCampaignsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            feedback: self.feedback,
            total_items: self.total_items,
        })
    }
}
