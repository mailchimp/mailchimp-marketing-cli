pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Top open locations for a specific campaign.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListLocationsReportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListLocationsReportsResponseLinksItem>>,
    /// The campaign id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign_id: Option<String>,
    /// An array of objects, each representing a top location for opens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<ListLocationsReportsResponseLocationsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListLocationsReportsResponse {
    pub fn builder() -> ListLocationsReportsResponseBuilder {
        <ListLocationsReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLocationsReportsResponseBuilder {
    links: Option<Vec<ListLocationsReportsResponseLinksItem>>,
    campaign_id: Option<String>,
    locations: Option<Vec<ListLocationsReportsResponseLocationsItem>>,
    total_items: Option<i64>,
}

impl ListLocationsReportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListLocationsReportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn campaign_id(mut self, value: impl Into<String>) -> Self {
        self.campaign_id = Some(value.into());
        self
    }

    pub fn locations(mut self, value: Vec<ListLocationsReportsResponseLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLocationsReportsResponse`].
    pub fn build(self) -> Result<ListLocationsReportsResponse, BuildError> {
        Ok(ListLocationsReportsResponse {
            links: self.links,
            campaign_id: self.campaign_id,
            locations: self.locations,
            total_items: self.total_items,
        })
    }
}
