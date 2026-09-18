pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The send checklist for the campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSendChecklistResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListSendChecklistResponseLinksItem>>,
    /// Whether the campaign is ready to send.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_ready: Option<bool>,
    /// A list of feedback items to review before sending your campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ListSendChecklistResponseItemsItem>>,
}

impl ListSendChecklistResponse {
    pub fn builder() -> ListSendChecklistResponseBuilder {
        <ListSendChecklistResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSendChecklistResponseBuilder {
    links: Option<Vec<ListSendChecklistResponseLinksItem>>,
    is_ready: Option<bool>,
    items: Option<Vec<ListSendChecklistResponseItemsItem>>,
}

impl ListSendChecklistResponseBuilder {
    pub fn links(mut self, value: Vec<ListSendChecklistResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn is_ready(mut self, value: bool) -> Self {
        self.is_ready = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<ListSendChecklistResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSendChecklistResponse`].
    pub fn build(self) -> Result<ListSendChecklistResponse, BuildError> {
        Ok(ListSendChecklistResponse {
            links: self.links,
            is_ready: self.is_ready,
            items: self.items,
        })
    }
}
