pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An array of objects, each representing an authorized application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAuthorizedAppsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListAuthorizedAppsResponseLinksItem>>,
    /// An array of objects, each representing an authorized application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<ListAuthorizedAppsResponseAppsItem>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListAuthorizedAppsResponse {
    pub fn builder() -> ListAuthorizedAppsResponseBuilder {
        <ListAuthorizedAppsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAuthorizedAppsResponseBuilder {
    links: Option<Vec<ListAuthorizedAppsResponseLinksItem>>,
    apps: Option<Vec<ListAuthorizedAppsResponseAppsItem>>,
    total_items: Option<i64>,
}

impl ListAuthorizedAppsResponseBuilder {
    pub fn links(mut self, value: Vec<ListAuthorizedAppsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn apps(mut self, value: Vec<ListAuthorizedAppsResponseAppsItem>) -> Self {
        self.apps = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAuthorizedAppsResponse`].
    pub fn build(self) -> Result<ListAuthorizedAppsResponse, BuildError> {
        Ok(ListAuthorizedAppsResponse {
            links: self.links,
            apps: self.apps,
            total_items: self.total_items,
        })
    }
}
