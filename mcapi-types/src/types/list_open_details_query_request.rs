pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-open-details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOpenDetailsQueryRequest {
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
    /// Restrict results to campaign open events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    /// Returns open reports sorted by the specified field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListOpenDetailsReportsRequestSortField>,
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<ListOpenDetailsReportsRequestSortDir>,
    /// When true, exclude automated (proxy/bot) opens so the returned open counts reflect human opens only, matching the in-app Recipient Activity view. A member whose opens are all automated is excluded from the human-only view. Defaults to false (all opens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_bots: Option<bool>,
}

impl ListOpenDetailsQueryRequest {
    pub fn builder() -> ListOpenDetailsQueryRequestBuilder {
        <ListOpenDetailsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOpenDetailsQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    since: Option<String>,
    sort_field: Option<ListOpenDetailsReportsRequestSortField>,
    sort_dir: Option<ListOpenDetailsReportsRequestSortDir>,
    filter_bots: Option<bool>,
}

impl ListOpenDetailsQueryRequestBuilder {
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

    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    pub fn sort_field(mut self, value: ListOpenDetailsReportsRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_dir(mut self, value: ListOpenDetailsReportsRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    pub fn filter_bots(mut self, value: bool) -> Self {
        self.filter_bots = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOpenDetailsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListOpenDetailsQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListOpenDetailsQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListOpenDetailsQueryRequest, BuildError> {
        Ok(ListOpenDetailsQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            since: self.since,
            sort_field: self.sort_field,
            sort_dir: self.sort_dir,
            filter_bots: self.filter_bots,
        })
    }
}

