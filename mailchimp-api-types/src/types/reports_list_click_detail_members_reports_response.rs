pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of members who clicked on a specific link within a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListClickDetailMembersReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListClickDetailMembersReportsResponseLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// An array of objects, each representing a member who clicked a specific link within a campaign.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ClickDetailMember>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListClickDetailMembersReportsResponse {
    pub fn builder() -> ListClickDetailMembersReportsResponseBuilder {
        <ListClickDetailMembersReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClickDetailMembersReportsResponseBuilder {
    links: Option<Vec<ListClickDetailMembersReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    members: Option<Vec<ClickDetailMember>>,
    total_items: Option<i64>,
}

impl ListClickDetailMembersReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListClickDetailMembersReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<ClickDetailMember>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListClickDetailMembersReportsResponse`].
    pub fn build(self) -> Result<ListClickDetailMembersReportsResponse, BuildError> {
        Ok(ListClickDetailMembersReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            members: self.members,
            total_items: self.total_items,
        })
    }
}
