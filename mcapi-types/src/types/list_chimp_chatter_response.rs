pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChimpChatterResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListChimpChatterResponseLinksItem>>,
    /// An array of Chimp Chatter messages. There's a maximum of 200 messages present for an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chimp_chatter: Option<Vec<ListChimpChatterResponseChimpChatterItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListChimpChatterResponse {
    pub fn builder() -> ListChimpChatterResponseBuilder {
        <ListChimpChatterResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChimpChatterResponseBuilder {
    links: Option<Vec<ListChimpChatterResponseLinksItem>>,
    chimp_chatter: Option<Vec<ListChimpChatterResponseChimpChatterItem>>,
    total_items: Option<i64>,
}

impl ListChimpChatterResponseBuilder {
    pub fn links(mut self, value: Vec<ListChimpChatterResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn chimp_chatter(mut self, value: Vec<ListChimpChatterResponseChimpChatterItem>) -> Self {
        self.chimp_chatter = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChimpChatterResponse`].
    pub fn build(self) -> Result<ListChimpChatterResponse, BuildError> {
        Ok(ListChimpChatterResponse {
            links: self.links,
            chimp_chatter: self.chimp_chatter,
            total_items: self.total_items,
        })
    }
}
