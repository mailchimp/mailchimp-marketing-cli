pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TemplatesListQueryRequest {
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
    /// The Mailchimp account user who created the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// Restrict the response to templates created after the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since_date_created: Option<String>,
    /// Restrict the response to templates created before the set date. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before_date_created: Option<String>,
    /// Limit results based on template type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Limit results based on category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The unique folder id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// Returns user templates sorted by the specified field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListTemplatesRequestSortField>,
    /// Limit results based on how the template's content is put together. Only templates of type `user` can be filtered by `content_type`. If you want to retrieve saved templates created with the legacy email editor, then filter `content_type` to `template`. If you'd rather pull your saved templates for the new editor, filter to `multichannel`. For code your own templates, filter to `html`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<ListTemplatesRequestContentType>,
    /// Determines the order direction for sorted results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<ListTemplatesRequestSortDir>,
}

impl TemplatesListQueryRequest {
    pub fn builder() -> TemplatesListQueryRequestBuilder {
        <TemplatesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplatesListQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    created_by: Option<String>,
    since_date_created: Option<String>,
    before_date_created: Option<String>,
    r#type: Option<String>,
    category: Option<String>,
    folder_id: Option<String>,
    sort_field: Option<ListTemplatesRequestSortField>,
    content_type: Option<ListTemplatesRequestContentType>,
    sort_dir: Option<ListTemplatesRequestSortDir>,
}

impl TemplatesListQueryRequestBuilder {
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

    pub fn since_date_created(mut self, value: impl Into<String>) -> Self {
        self.since_date_created = Some(value.into());
        self
    }

    pub fn before_date_created(mut self, value: impl Into<String>) -> Self {
        self.before_date_created = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn folder_id(mut self, value: impl Into<String>) -> Self {
        self.folder_id = Some(value.into());
        self
    }

    pub fn sort_field(mut self, value: ListTemplatesRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    pub fn content_type(mut self, value: ListTemplatesRequestContentType) -> Self {
        self.content_type = Some(value);
        self
    }

    pub fn sort_dir(mut self, value: ListTemplatesRequestSortDir) -> Self {
        self.sort_dir = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplatesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](TemplatesListQueryRequestBuilder::fields)
    /// - [`exclude_fields`](TemplatesListQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<TemplatesListQueryRequest, BuildError> {
        Ok(TemplatesListQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            created_by: self.created_by,
            since_date_created: self.since_date_created,
            before_date_created: self.before_date_created,
            r#type: self.r#type,
            category: self.category,
            folder_id: self.folder_id,
            sort_field: self.sort_field,
            content_type: self.content_type,
            sort_dir: self.sort_dir,
        })
    }
}

