pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of connected sites in the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListConnectedSitesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListConnectedSitesResponseLinksItem>>,
    /// An array of objects, each representing a connected site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sites: Option<Vec<ConnectedSite>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListConnectedSitesResponse {
    pub fn builder() -> ListConnectedSitesResponseBuilder {
        <ListConnectedSitesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListConnectedSitesResponseBuilder {
    links: Option<Vec<ListConnectedSitesResponseLinksItem>>,
    sites: Option<Vec<ConnectedSite>>,
    total_items: Option<i64>,
}

impl ListConnectedSitesResponseBuilder {
    pub fn links(mut self, value: Vec<ListConnectedSitesResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn sites(mut self, value: Vec<ConnectedSite>) -> Self {
        self.sites = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListConnectedSitesResponse`].
    pub fn build(self) -> Result<ListConnectedSitesResponse, BuildError> {
        Ok(ListConnectedSitesResponse {
            links: self.links,
            sites: self.sites,
            total_items: self.total_items,
        })
    }
}
