pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about this list's interest categories.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListInterestCategoriesListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListInterestCategoriesListsResponseLinksItem>>,
    /// This array contains individual interest categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<InterestCategory>>,
    /// The ID for the list that this category belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListInterestCategoriesListsResponse {
    pub fn builder() -> ListInterestCategoriesListsResponseBuilder {
        <ListInterestCategoriesListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInterestCategoriesListsResponseBuilder {
    links: Option<Vec<ListInterestCategoriesListsResponseLinksItem>>,
    categories: Option<Vec<InterestCategory>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListInterestCategoriesListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListInterestCategoriesListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn categories(mut self, value: Vec<InterestCategory>) -> Self {
        self.categories = Some(value);
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

    /// Consumes the builder and constructs a [`ListInterestCategoriesListsResponse`].
    pub fn build(self) -> Result<ListInterestCategoriesListsResponse, BuildError> {
        Ok(ListInterestCategoriesListsResponse {
            links: self.links,
            categories: self.categories,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
