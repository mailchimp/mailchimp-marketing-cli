pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Any request query parameters. Example parameters: {"count":10, "offset":0}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRequestOperationsItemParams {
}

impl CreateRequestOperationsItemParams {
    pub fn builder() -> CreateRequestOperationsItemParamsBuilder {
        <CreateRequestOperationsItemParamsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestOperationsItemParamsBuilder {
}

impl CreateRequestOperationsItemParamsBuilder {

    /// Consumes the builder and constructs a [`CreateRequestOperationsItemParams`].
    pub fn build(self) -> Result<CreateRequestOperationsItemParams, BuildError> {
        Ok(CreateRequestOperationsItemParams {
        })
    }
}
