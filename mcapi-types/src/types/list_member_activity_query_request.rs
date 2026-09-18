pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-member-activity
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListMemberActivityQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// A comma seperated list of actions to return.
    #[serde(default)]
    pub action: Vec<Option<ListMemberActivityRequestActionItem>>,
}

impl ListMemberActivityQueryRequest {
    pub fn builder() -> ListMemberActivityQueryRequestBuilder {
        <ListMemberActivityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListMemberActivityQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    action: Option<Vec<Option<ListMemberActivityRequestActionItem>>>,
}

impl ListMemberActivityQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn action(mut self, value: Vec<Option<ListMemberActivityRequestActionItem>>) -> Self {
        self.action = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListMemberActivityQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListMemberActivityQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListMemberActivityQueryRequestBuilder::exclude_fields)
    /// - [`action`](ListMemberActivityQueryRequestBuilder::action)
    pub fn build(self) -> Result<ListMemberActivityQueryRequest, BuildError> {
        Ok(ListMemberActivityQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
        })
    }
}

