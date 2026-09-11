pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBatchesRequest {
    /// An array of objects that describes operations to perform.
    #[serde(default)]
    pub operations: Vec<CreateBatchesRequestOperationsItem>,
}

impl CreateBatchesRequest {
    pub fn builder() -> CreateBatchesRequestBuilder {
        <CreateBatchesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchesRequestBuilder {
    operations: Option<Vec<CreateBatchesRequestOperationsItem>>,
}

impl CreateBatchesRequestBuilder {
    pub fn operations(mut self, value: Vec<CreateBatchesRequestOperationsItem>) -> Self {
        self.operations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateBatchesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operations`](CreateBatchesRequestBuilder::operations)
    pub fn build(self) -> Result<CreateBatchesRequest, BuildError> {
        Ok(CreateBatchesRequest {
            operations: self.operations.ok_or_else(|| BuildError::missing_field("operations"))?,
        })
    }
}

