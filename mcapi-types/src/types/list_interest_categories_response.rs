pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about this list's interest categories.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListInterestCategoriesResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListInterestCategoriesResponseLinksItem>>,
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

impl ListInterestCategoriesResponse {
    pub fn builder() -> ListInterestCategoriesResponseBuilder {
        <ListInterestCategoriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInterestCategoriesResponseBuilder {
    links: Option<Vec<ListInterestCategoriesResponseLinksItem>>,
    categories: Option<Vec<InterestCategory>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListInterestCategoriesResponseBuilder {
    pub fn links(mut self, value: Vec<ListInterestCategoriesResponseLinksItem>) -> Self {
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

    /// Consumes the builder and constructs a [`ListInterestCategoriesResponse`].
    pub fn build(self) -> Result<ListInterestCategoriesResponse, BuildError> {
        Ok(ListInterestCategoriesResponse {
            links: self.links,
            categories: self.categories,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
