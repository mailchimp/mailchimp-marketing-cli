pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Any HTTP headers to include with the request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateBatchesRequestOperationsItemHeaders {
}

impl CreateBatchesRequestOperationsItemHeaders {
    pub fn builder() -> CreateBatchesRequestOperationsItemHeadersBuilder {
        <CreateBatchesRequestOperationsItemHeadersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBatchesRequestOperationsItemHeadersBuilder {
}

impl CreateBatchesRequestOperationsItemHeadersBuilder {

    /// Consumes the builder and constructs a [`CreateBatchesRequestOperationsItemHeaders`].
    pub fn build(self) -> Result<CreateBatchesRequestOperationsItemHeaders, BuildError> {
        Ok(CreateBatchesRequestOperationsItemHeaders {
        })
    }
}
