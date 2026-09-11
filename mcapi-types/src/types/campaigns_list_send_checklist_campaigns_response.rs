pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The send checklist for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSendChecklistCampaignsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSendChecklistCampaignsResponseLinksItem>>,
    /// Whether the campaign is ready to send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ready: Option<bool>,
    /// A list of feedback items to review before sending your campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ListSendChecklistCampaignsResponseItemsItem>>,
}

impl ListSendChecklistCampaignsResponse {
    pub fn builder() -> ListSendChecklistCampaignsResponseBuilder {
        <ListSendChecklistCampaignsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSendChecklistCampaignsResponseBuilder {
    links: Option<Vec<ListSendChecklistCampaignsResponseLinksItem>>,
    is_ready: Option<bool>,
    items: Option<Vec<ListSendChecklistCampaignsResponseItemsItem>>,
}

impl ListSendChecklistCampaignsResponseBuilder {
    pub fn links(mut self, value: Vec<ListSendChecklistCampaignsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn is_ready(mut self, value: bool) -> Self {
        self.is_ready = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<ListSendChecklistCampaignsResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSendChecklistCampaignsResponse`].
    pub fn build(self) -> Result<ListSendChecklistCampaignsResponse, BuildError> {
        Ok(ListSendChecklistCampaignsResponse {
            links: self.links,
            is_ready: self.is_ready,
            items: self.items,
        })
    }
}
