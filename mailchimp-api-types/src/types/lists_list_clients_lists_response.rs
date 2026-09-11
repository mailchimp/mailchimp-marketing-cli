pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The top email clients based on user-agent strings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListClientsListsResponseLinksItem>>,
    /// An array of top email clients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<ListClientsListsResponseClientsItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListClientsListsResponse {
    pub fn builder() -> ListClientsListsResponseBuilder {
        <ListClientsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientsListsResponseBuilder {
    links: Option<Vec<ListClientsListsResponseLinksItem>>,
    clients: Option<Vec<ListClientsListsResponseClientsItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListClientsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListClientsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn clients(mut self, value: Vec<ListClientsListsResponseClientsItem>) -> Self {
        self.clients = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListClientsListsResponse`].
    pub fn build(self) -> Result<ListClientsListsResponse, BuildError> {
        Ok(ListClientsListsResponse {
            links: self.links,
            clients: self.clients,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
