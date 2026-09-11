pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Any request query parameters. Example parameters: {"count":10, "offset":0}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBatchesRequestOperationsItemParams {
}

impl CreateBatchesRequestOperationsItemParams {
    pub fn builder() -> CreateBatchesRequestOperationsItemParamsBuilder {
        <CreateBatchesRequestOperationsItemParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchesRequestOperationsItemParamsBuilder {
}

impl CreateBatchesRequestOperationsItemParamsBuilder {

    /// Consumes the builder and constructs a [`CreateBatchesRequestOperationsItemParams`].
    pub fn build(self) -> Result<CreateBatchesRequestOperationsItemParams, BuildError> {
        Ok(CreateBatchesRequestOperationsItemParams {
        })
    }
}
