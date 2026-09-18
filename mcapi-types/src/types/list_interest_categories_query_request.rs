pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-interest-categories
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListInterestCategoriesQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this is the number of records from a collection to skip. Default value is 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Restrict results a type of interest group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Returns interest categories sorted by the specified field. Defaults to display_order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListInterestCategoriesRequestSortField>,
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<ListInterestCategoriesRequestSortDir>,
}

impl ListInterestCategoriesQueryRequest {
    pub fn builder() -> ListInterestCategoriesQueryRequestBuilder {
        <ListInterestCategoriesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListInterestCategoriesQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    r#type: Option<String>,
    sort_field: Option<ListInterestCategoriesRequestSortField>,
    sort_dir: Option<ListInterestCategoriesRequestSortDir>,
}

impl ListInterestCategoriesQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn sort_field(mut self, value: ListInterestCategoriesRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_dir(mut self, value: ListInterestCategoriesRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListInterestCategoriesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListInterestCategoriesQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListInterestCategoriesQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListInterestCategoriesQueryRequest, BuildError> {
        Ok(ListInterestCategoriesQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            r#type: self.r#type,
            sort_field: self.sort_field,
            sort_dir: self.sort_dir,
        })
    }
}

