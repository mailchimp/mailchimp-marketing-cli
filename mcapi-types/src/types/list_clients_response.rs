pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The top email clients based on user-agent strings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListClientsResponseLinksItem>>,
    /// An array of top email clients.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<ListClientsResponseClientsItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListClientsResponse {
    pub fn builder() -> ListClientsResponseBuilder {
        <ListClientsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientsResponseBuilder {
    links: Option<Vec<ListClientsResponseLinksItem>>,
    clients: Option<Vec<ListClientsResponseClientsItem>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListClientsResponseBuilder {
    pub fn links(mut self, value: Vec<ListClientsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn clients(mut self, value: Vec<ListClientsResponseClientsItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListClientsResponse`].
    pub fn build(self) -> Result<ListClientsResponse, BuildError> {
        Ok(ListClientsResponse {
            links: self.links,
            clients: self.clients,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
