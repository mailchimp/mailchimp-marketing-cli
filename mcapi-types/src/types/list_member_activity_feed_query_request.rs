pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-member-activity-feed
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberActivityFeedQueryRequest {
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
    /// A comma-separated list of activity filters that correspond to a set of activity types, e.g "?activity_filters=open,bounce,click".
    #[serde(default)]
    pub activity_filters: Vec<Option<ListMemberActivityFeedListsRequestActivityFiltersItem>>,
}

impl ListMemberActivityFeedQueryRequest {
    pub fn builder() -> ListMemberActivityFeedQueryRequestBuilder {
        <ListMemberActivityFeedQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberActivityFeedQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    activity_filters: Option<Vec<Option<ListMemberActivityFeedListsRequestActivityFiltersItem>>>,
}

impl ListMemberActivityFeedQueryRequestBuilder {
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

    pub fn activity_filters(mut self, value: Vec<Option<ListMemberActivityFeedListsRequestActivityFiltersItem>>) -> Self {
        self.activity_filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberActivityFeedQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListMemberActivityFeedQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListMemberActivityFeedQueryRequestBuilder::exclude_fields)
    /// - [`activity_filters`](ListMemberActivityFeedQueryRequestBuilder::activity_filters)
    pub fn build(self) -> Result<ListMemberActivityFeedQueryRequest, BuildError> {
        Ok(ListMemberActivityFeedQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            activity_filters: self.activity_filters.ok_or_else(|| BuildError::missing_field("activity_filters"))?,
        })
    }
}

