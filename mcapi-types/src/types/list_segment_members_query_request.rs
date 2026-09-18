pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-segment-members
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSegmentMembersQueryRequest {
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
    /// Include cleaned members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_cleaned: Option<bool>,
    /// Include transactional members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transactional: Option<bool>,
    /// Include unsubscribed members in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_unsubscribed: Option<bool>,
}

impl ListSegmentMembersQueryRequest {
    pub fn builder() -> ListSegmentMembersQueryRequestBuilder {
        <ListSegmentMembersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSegmentMembersQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    include_cleaned: Option<bool>,
    include_transactional: Option<bool>,
    include_unsubscribed: Option<bool>,
}

impl ListSegmentMembersQueryRequestBuilder {
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

    pub fn include_cleaned(mut self, value: bool) -> Self {
        self.include_cleaned = Some(value);
        self
    }

    pub fn include_transactional(mut self, value: bool) -> Self {
        self.include_transactional = Some(value);
        self
    }

    pub fn include_unsubscribed(mut self, value: bool) -> Self {
        self.include_unsubscribed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSegmentMembersQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListSegmentMembersQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListSegmentMembersQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListSegmentMembersQueryRequest, BuildError> {
        Ok(ListSegmentMembersQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            include_cleaned: self.include_cleaned,
            include_transactional: self.include_transactional,
            include_unsubscribed: self.include_unsubscribed,
        })
    }
}

