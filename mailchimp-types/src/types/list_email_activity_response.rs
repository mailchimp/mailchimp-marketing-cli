pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of member's subscriber activity in a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEmailActivityResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListEmailActivityResponseLinksItem>>,
    /// The unique id for the sent campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// An array of members that were sent the campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<EmailActivity>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListEmailActivityResponse {
    pub fn builder() -> ListEmailActivityResponseBuilder {
        <ListEmailActivityResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEmailActivityResponseBuilder {
    links: Option<Vec<ListEmailActivityResponseLinksItem>>,
    campaign_id: Option<String>,
    emails: Option<Vec<EmailActivity>>,
    total_items: Option<i64>,
}

impl ListEmailActivityResponseBuilder {
    pub fn links(mut self, value: Vec<ListEmailActivityResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn emails(mut self, value: Vec<EmailActivity>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEmailActivityResponse`].
    pub fn build(self) -> Result<ListEmailActivityResponse, BuildError> {
        Ok(ListEmailActivityResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            emails: self.emails,
            total_items: self.total_items,
        })
    }
}
