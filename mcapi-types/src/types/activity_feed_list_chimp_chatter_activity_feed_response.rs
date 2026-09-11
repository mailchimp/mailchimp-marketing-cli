pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChimpChatterActivityFeedResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListChimpChatterActivityFeedResponseLinksItem>>,
    /// An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chimp_chatter: Option<Vec<ListChimpChatterActivityFeedResponseChimpChatterItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListChimpChatterActivityFeedResponse {
    pub fn builder() -> ListChimpChatterActivityFeedResponseBuilder {
        <ListChimpChatterActivityFeedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChimpChatterActivityFeedResponseBuilder {
    links: Option<Vec<ListChimpChatterActivityFeedResponseLinksItem>>,
    chimp_chatter: Option<Vec<ListChimpChatterActivityFeedResponseChimpChatterItem>>,
    total_items: Option<i64>,
}

impl ListChimpChatterActivityFeedResponseBuilder {
    pub fn links(mut self, value: Vec<ListChimpChatterActivityFeedResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn chimp_chatter(mut self, value: Vec<ListChimpChatterActivityFeedResponseChimpChatterItem>) -> Self {
        self.chimp_chatter = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChimpChatterActivityFeedResponse`].
    pub fn build(self) -> Result<ListChimpChatterActivityFeedResponse, BuildError> {
        Ok(ListChimpChatterActivityFeedResponse {
            links: self.links,
            chimp_chatter: self.chimp_chatter,
            total_items: self.total_items,
        })
    }
}
