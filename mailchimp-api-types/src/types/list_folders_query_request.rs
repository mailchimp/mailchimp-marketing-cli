pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-folders
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListFoldersQueryRequest {
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
    /// The Mailchimp account user who created the File Manager file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Restrict the response to files created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_created_at: Option<String>,
    /// Restrict the response to files created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_created_at: Option<String>,
}

impl ListFoldersQueryRequest {
    pub fn builder() -> ListFoldersQueryRequestBuilder {
        <ListFoldersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListFoldersQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    created_by: Option<String>,
    before_created_at: Option<String>,
    since_created_at: Option<String>,
}

impl ListFoldersQueryRequestBuilder {
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

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn before_created_at(mut self, value: impl Into<String>) -> Self {
        self.before_created_at = Some(value.into());
        self
    }

    pub fn since_created_at(mut self, value: impl Into<String>) -> Self {
        self.since_created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListFoldersQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListFoldersQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListFoldersQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListFoldersQueryRequest, BuildError> {
        Ok(ListFoldersQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            created_by: self.created_by,
            before_created_at: self.before_created_at,
            since_created_at: self.since_created_at,
        })
    }
}

