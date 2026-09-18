pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The [merge fields](https://mailchimp.com/developer/marketing/docs/merge-fields/) for an audience.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMergeFieldsResponse {
    /// A list of link types and descriptions for the API schema documents.
    #[serde(rename = "_links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<ListMergeFieldsResponseLinksItem>>,
    /// The list id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    /// The maximum number of merge fields this audience can hold. The limit is determined by the account's plan. Subtract `total_items` from this value to derive the remaining capacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_field_limit: Option<i64>,
    /// An array of objects, each representing a merge field resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_fields: Option<Vec<MergeField>>,
    /// The total number of items matching the query regardless of pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_items: Option<i64>,
}

impl ListMergeFieldsResponse {
    pub fn builder() -> ListMergeFieldsResponseBuilder {
        <ListMergeFieldsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMergeFieldsResponseBuilder {
    links: Option<Vec<ListMergeFieldsResponseLinksItem>>,
    list_id: Option<String>,
    merge_field_limit: Option<i64>,
    merge_fields: Option<Vec<MergeField>>,
    total_items: Option<i64>,
}

impl ListMergeFieldsResponseBuilder {
    pub fn links(mut self, value: Vec<ListMergeFieldsResponseLinksItem>) -> Self {
        self.links = Some(value);
        self
    }

    pub fn list_id(mut self, value: impl Into<String>) -> Self {
        self.list_id = Some(value.into());
        self
    }

    pub fn merge_field_limit(mut self, value: i64) -> Self {
        self.merge_field_limit = Some(value);
        self
    }

    pub fn merge_fields(mut self, value: Vec<MergeField>) -> Self {
        self.merge_fields = Some(value);
        self
    }

    pub fn total_items(mut self, value: i64) -> Self {
        self.total_items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMergeFieldsResponse`].
    pub fn build(self) -> Result<ListMergeFieldsResponse, BuildError> {
        Ok(ListMergeFieldsResponse {
            links: self.links,
            list_id: self.list_id,
            merge_field_limit: self.merge_field_limit,
            merge_fields: self.merge_fields,
            total_items: self.total_items,
        })
    }
}
