pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The verified domains currently on the account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVerifiedDomainsResponse {
    /// The domains on the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<ListVerifiedDomainsResponseDomainsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListVerifiedDomainsResponse {
    pub fn builder() -> ListVerifiedDomainsResponseBuilder {
        <ListVerifiedDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerifiedDomainsResponseBuilder {
    domains: Option<Vec<ListVerifiedDomainsResponseDomainsItem>>,
    total_items: Option<i64>,
}

impl ListVerifiedDomainsResponseBuilder {
    pub fn domains(mut self, value: Vec<ListVerifiedDomainsResponseDomainsItem>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVerifiedDomainsResponse`].
    pub fn build(self) -> Result<ListVerifiedDomainsResponse, BuildError> {
        Ok(ListVerifiedDomainsResponse {
            domains: self.domains,
            total_items: self.total_items,
        })
    }
}
