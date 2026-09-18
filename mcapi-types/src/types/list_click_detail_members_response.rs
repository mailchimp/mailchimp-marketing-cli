pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of members who clicked on a specific link within a campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListClickDetailMembersResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListClickDetailMembersResponseLinksItem>>,
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

impl ListClickDetailMembersResponse {
    pub fn builder() -> ListClickDetailMembersResponseBuilder {
        <ListClickDetailMembersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClickDetailMembersResponseBuilder {
    links: Option<Vec<ListClickDetailMembersResponseLinksItem>>,
    campaign_id: Option<String>,
    members: Option<Vec<ClickDetailMember>>,
    total_items: Option<i64>,
}

impl ListClickDetailMembersResponseBuilder {
    pub fn links(mut self, value: Vec<ListClickDetailMembersResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListClickDetailMembersResponse`].
    pub fn build(self) -> Result<ListClickDetailMembersResponse, BuildError> {
        Ok(ListClickDetailMembersResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            members: self.members,
            total_items: self.total_items,
        })
    }
}
