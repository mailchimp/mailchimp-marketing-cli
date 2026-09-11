pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of subscriber lists for this account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListListsResponseLinksItem>>,
    /// Do particular authorization constraints around this collection limit creation of new instances?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constraints: Option<ListListsResponseConstraints>,
    /// An array of objects, each representing a list.
    #[serde(default)]
    pub lists: Vec<SubscriberList>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListListsResponse {
    pub fn builder() -> ListListsResponseBuilder {
        <ListListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListListsResponseBuilder {
    links: Option<Vec<ListListsResponseLinksItem>>,
    constraints: Option<ListListsResponseConstraints>,
    lists: Option<Vec<SubscriberList>>,
    total_items: Option<i64>,
}

impl ListListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn constraints(mut self, value: ListListsResponseConstraints) -> Self {
        self.constraints = Some(value);
        self
    }

    pub fn lists(mut self, value: Vec<SubscriberList>) -> Self {
        self.lists = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListListsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`lists`](ListListsResponseBuilder::lists)
    pub fn build(self) -> Result<ListListsResponse, BuildError> {
        Ok(ListListsResponse {
            links: self.links,
            constraints: self.constraints,
            lists: self.lists.ok_or_else(|| BuildError::missing_field("lists"))?,
            total_items: self.total_items,
        })
    }
}
