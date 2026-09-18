pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Any HTTP headers to include with the request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateRequestOperationsItemHeaders {
}

impl CreateRequestOperationsItemHeaders {
    pub fn builder() -> CreateRequestOperationsItemHeadersBuilder {
        <CreateRequestOperationsItemHeadersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestOperationsItemHeadersBuilder {
}

impl CreateRequestOperationsItemHeadersBuilder {

    /// Consumes the builder and constructs a [`CreateRequestOperationsItemHeaders`].
    pub fn build(self) -> Result<CreateRequestOperationsItemHeaders, BuildError> {
        Ok(CreateRequestOperationsItemHeaders {
        })
    }
}
