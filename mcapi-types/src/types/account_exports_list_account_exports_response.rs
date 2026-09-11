pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An array of objects, each representing an account export.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAccountExportsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAccountExportsResponseLinksItem>>,
    /// An array of objects, each representing an account export.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports: Option<Vec<ListAccountExportsResponseExportsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListAccountExportsResponse {
    pub fn builder() -> ListAccountExportsResponseBuilder {
        <ListAccountExportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAccountExportsResponseBuilder {
    links: Option<Vec<ListAccountExportsResponseLinksItem>>,
    exports: Option<Vec<ListAccountExportsResponseExportsItem>>,
    total_items: Option<i64>,
}

impl ListAccountExportsResponseBuilder {
    pub fn links(mut self, value: Vec<ListAccountExportsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn exports(mut self, value: Vec<ListAccountExportsResponseExportsItem>) -> Self {
        self.exports = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAccountExportsResponse`].
    pub fn build(self) -> Result<ListAccountExportsResponse, BuildError> {
        Ok(ListAccountExportsResponse {
            links: self.links,
            exports: self.exports,
            total_items: self.total_items,
        })
    }
}
