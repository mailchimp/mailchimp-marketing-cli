pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of the comment feedback for a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFeedbackResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListFeedbackResponseLinksItem>>,
    /// The unique id for the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// A collection of feedback items for a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<Vec<ListFeedbackResponseFeedbackItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListFeedbackResponse {
    pub fn builder() -> ListFeedbackResponseBuilder {
        <ListFeedbackResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFeedbackResponseBuilder {
    links: Option<Vec<ListFeedbackResponseLinksItem>>,
    campaign_id: Option<String>,
    feedback: Option<Vec<ListFeedbackResponseFeedbackItem>>,
    total_items: Option<i64>,
}

impl ListFeedbackResponseBuilder {
    pub fn links(mut self, value: Vec<ListFeedbackResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn feedback(mut self, value: Vec<ListFeedbackResponseFeedbackItem>) -> Self {
        self.feedback = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListFeedbackResponse`].
    pub fn build(self) -> Result<ListFeedbackResponse, BuildError> {
        Ok(ListFeedbackResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            feedback: self.feedback,
            total_items: self.total_items,
        })
    }
}
