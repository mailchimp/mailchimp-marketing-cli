pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A summary of List's locations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListLocationsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListLocationsListsResponseLinksItem>>,
    /// The unique id for the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// An array of objects, each representing a list's top subscriber locations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<ListLocationsListsResponseLocationsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListLocationsListsResponse {
    pub fn builder() -> ListLocationsListsResponseBuilder {
        <ListLocationsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListLocationsListsResponseBuilder {
    links: Option<Vec<ListLocationsListsResponseLinksItem>>,
    list_id: Option<String>,
    locations: Option<Vec<ListLocationsListsResponseLocationsItem>>,
    total_items: Option<i64>,
}

impl ListLocationsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListLocationsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn locations(mut self, value: Vec<ListLocationsListsResponseLocationsItem>) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListLocationsListsResponse`].
    pub fn build(self) -> Result<ListLocationsListsResponse, BuildError> {
        Ok(ListLocationsListsResponse {
            links: self.links,
            list_id: self.list_id,
            locations: self.locations,
            total_items: self.total_items,
        })
    }
}
