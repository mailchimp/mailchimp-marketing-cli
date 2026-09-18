pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The verified domains currently on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListResponse {
    /// The domains on the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<ListResponseDomainsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListResponse {
    pub fn builder() -> ListResponseBuilder {
        <ListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListResponseBuilder {
    domains: Option<Vec<ListResponseDomainsItem>>,
    total_items: Option<i64>,
}

impl ListResponseBuilder {
    pub fn domains(mut self, value: Vec<ListResponseDomainsItem>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListResponse`].
    pub fn build(self) -> Result<ListResponse, BuildError> {
        Ok(ListResponse {
            domains: self.domains,
            total_items: self.total_items,
        })
    }
}
