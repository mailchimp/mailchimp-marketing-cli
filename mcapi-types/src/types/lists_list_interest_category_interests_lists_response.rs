pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A list of this category's interests
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListInterestCategoryInterestsListsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListInterestCategoryInterestsListsResponseLinksItem>>,
    /// The id for the interest category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// An array of this category's interests
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<Interest>>,
    /// The unique list id that the interests belong to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListInterestCategoryInterestsListsResponse {
    pub fn builder() -> ListInterestCategoryInterestsListsResponseBuilder {
        <ListInterestCategoryInterestsListsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInterestCategoryInterestsListsResponseBuilder {
    links: Option<Vec<ListInterestCategoryInterestsListsResponseLinksItem>>,
    category_id: Option<String>,
    interests: Option<Vec<Interest>>,
    list_id: Option<String>,
    total_items: Option<i64>,
}

impl ListInterestCategoryInterestsListsResponseBuilder {
    pub fn links(mut self, value: Vec<ListInterestCategoryInterestsListsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn category_id(mut self, value: impl Into<String>) -> Self {
        self.category_id = Some(value.into());
        self
    }

    pub fn interests(mut self, value: Vec<Interest>) -> Self {
        self.interests = Some(value);
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

    /// Consumes the builder and constructs a [`ListInterestCategoryInterestsListsResponse`].
    pub fn build(self) -> Result<ListInterestCategoryInterestsListsResponse, BuildError> {
        Ok(ListInterestCategoryInterestsListsResponse {
            links: self.links,
            category_id: self.category_id,
            interests: self.interests,
            list_id: self.list_id,
            total_items: self.total_items,
        })
    }
}
