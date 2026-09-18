pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-ecommerce-product-activity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEcommerceProductActivityQueryRequest {
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
    /// Returns files sorted by the specified field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<ListEcommerceProductActivityRequestSortField>,
}

impl ListEcommerceProductActivityQueryRequest {
    pub fn builder() -> ListEcommerceProductActivityQueryRequestBuilder {
        <ListEcommerceProductActivityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEcommerceProductActivityQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    count: Option<i64>,
    offset: Option<i64>,
    sort_field: Option<ListEcommerceProductActivityRequestSortField>,
}

impl ListEcommerceProductActivityQueryRequestBuilder {
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

    pub fn sort_field(mut self, value: ListEcommerceProductActivityRequestSortField) -> Self {
        self.sort_field = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEcommerceProductActivityQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListEcommerceProductActivityQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListEcommerceProductActivityQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListEcommerceProductActivityQueryRequest, BuildError> {
        Ok(ListEcommerceProductActivityQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            count: self.count,
            offset: self.offset,
            sort_field: self.sort_field,
        })
    }
}

