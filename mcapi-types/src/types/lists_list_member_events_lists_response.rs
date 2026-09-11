pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of events for a given contact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMemberEventsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberEventsListsResponseLinksItem>>,
    /// An array of objects, each representing an event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ListMemberEventsListsResponseEventsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberEventsListsResponse {
    pub fn builder() -> ListMemberEventsListsResponseBuilder {
        <ListMemberEventsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberEventsListsResponseBuilder {
    links: Option<Vec<ListMemberEventsListsResponseLinksItem>>,
    events: Option<Vec<ListMemberEventsListsResponseEventsItem>>,
    total_items: Option<i64>,
}

impl ListMemberEventsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberEventsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<ListMemberEventsListsResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberEventsListsResponse`].
    pub fn build(self) -> Result<ListMemberEventsListsResponse, BuildError> {
        Ok(ListMemberEventsListsResponse {
            links: self.links,
            events: self.events,
            total_items: self.total_items,
        })
    }
}
