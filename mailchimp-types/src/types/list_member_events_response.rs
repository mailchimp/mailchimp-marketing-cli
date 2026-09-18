pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A collection of events for a given contact
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListMemberEventsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMemberEventsResponseLinksItem>>,
    /// An array of objects, each representing an event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ListMemberEventsResponseEventsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMemberEventsResponse {
    pub fn builder() -> ListMemberEventsResponseBuilder {
        <ListMemberEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberEventsResponseBuilder {
    links: Option<Vec<ListMemberEventsResponseLinksItem>>,
    events: Option<Vec<ListMemberEventsResponseEventsItem>>,
    total_items: Option<i64>,
}

impl ListMemberEventsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMemberEventsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<ListMemberEventsResponseEventsItem>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberEventsResponse`].
    pub fn build(self) -> Result<ListMemberEventsResponse, BuildError> {
        Ok(ListMemberEventsResponse {
            links: self.links,
            events: self.events,
            total_items: self.total_items,
        })
    }
}
