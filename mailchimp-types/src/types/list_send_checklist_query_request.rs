pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list-send-checklist
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSendChecklistQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
}

impl ListSendChecklistQueryRequest {
    pub fn builder() -> ListSendChecklistQueryRequestBuilder {
        <ListSendChecklistQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSendChecklistQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
}

impl ListSendChecklistQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSendChecklistQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](ListSendChecklistQueryRequestBuilder::fields)
    /// - [`exclude_fields`](ListSendChecklistQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<ListSendChecklistQueryRequest, BuildError> {
        Ok(ListSendChecklistQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
        })
    }
}

