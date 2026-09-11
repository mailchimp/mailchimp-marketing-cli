pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The email client.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListClientsListsResponseClientsItem {
    /// The name of the email client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    /// The number of subscribed members who used this email client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<i64>,
}

impl ListClientsListsResponseClientsItem {
    pub fn builder() -> ListClientsListsResponseClientsItemBuilder {
        <ListClientsListsResponseClientsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListClientsListsResponseClientsItemBuilder {
    client: Option<String>,
    members: Option<i64>,
}

impl ListClientsListsResponseClientsItemBuilder {
    pub fn client(mut self, value: impl Into<String>) -> Self {
        self.client = Some(value.into());
        self
    }

    pub fn members(mut self, value: i64) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListClientsListsResponseClientsItem`].
    pub fn build(self) -> Result<ListClientsListsResponseClientsItem, BuildError> {
        Ok(ListClientsListsResponseClientsItem {
            client: self.client,
            members: self.members,
        })
    }
}
