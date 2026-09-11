pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListsGetQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// Deprecated. Return the total_contacts field in the stats response, which contains an approximate count of subscribed, unsubscribed, and transactional contacts. For a complete audience contact count, use the /audiences endpoint instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_contacts: Option<bool>,
}

impl ListsGetQueryRequest {
    pub fn builder() -> ListsGetQueryRequestBuilder {
        <ListsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListsGetQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    include_total_contacts: Option<bool>,
}

impl ListsGetQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn include_total_contacts(mut self, value: bool) -> Self {
        self.include_total_contacts = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListsGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListsGetQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListsGetQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListsGetQueryRequest, BuildError> {
        Ok(ListsGetQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            include_total_contacts: self.include_total_contacts,
        })
    }
}

