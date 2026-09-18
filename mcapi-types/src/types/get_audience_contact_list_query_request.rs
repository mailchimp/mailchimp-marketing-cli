pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getAudienceContactList
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAudienceContactListQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// The number of records to return. Default value is 10. Maximum value is 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Paginate through a collection of records by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request. Default value fetches the first "page" of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Restricts the response to contacts created at or before the specified time (inclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Restricts the response to contacts created after the specified time (exclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_since: Option<DateTime<FixedOffset>>,
    /// Restricts the response to contacts updated at or before the specified time (inclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_before: Option<DateTime<FixedOffset>>,
    /// Restricts the response to contacts updated after the specified time (exclusive). Uses ISO 8601 format: 2025-04-23T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub updated_since: Option<DateTime<FixedOffset>>,
    /// Specifies the field to sort the returned contacts by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<GetAudienceContactListRequestSortField>,
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<GetAudienceContactListRequestSortDir>,
}

impl GetAudienceContactListQueryRequest {
    pub fn builder() -> GetAudienceContactListQueryRequestBuilder {
        <GetAudienceContactListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAudienceContactListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    cursor: Option<String>,
    created_before: Option<DateTime<FixedOffset>>,
    created_since: Option<DateTime<FixedOffset>>,
    updated_before: Option<DateTime<FixedOffset>>,
    updated_since: Option<DateTime<FixedOffset>>,
    sort_field: Option<GetAudienceContactListRequestSortField>,
    sort_dir: Option<GetAudienceContactListRequestSortDir>,
}

impl GetAudienceContactListQueryRequestBuilder {
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

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_since = Some(value);
        self
    }

    pub fn updated_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_before = Some(value);
        self
    }

    pub fn updated_since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_since = Some(value);
        self
    }

    pub fn sort_field(mut self, value: GetAudienceContactListRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn sort_dir(mut self, value: GetAudienceContactListRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAudienceContactListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](GetAudienceContactListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](GetAudienceContactListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<GetAudienceContactListQueryRequest, BuildError> {
        Ok(GetAudienceContactListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            cursor: self.cursor,
            created_before: self.created_before,
            created_since: self.created_since,
            updated_before: self.updated_before,
            updated_since: self.updated_since,
            sort_field: self.sort_field,
            sort_dir: self.sort_dir,
        })
    }
}

