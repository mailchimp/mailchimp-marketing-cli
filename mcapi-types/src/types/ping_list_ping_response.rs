pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// API health status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPingResponse {
    /// This will return a constant string value if the request is successful. Ex. "Everything's Chimpy!"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
}

impl ListPingResponse {
    pub fn builder() -> ListPingResponseBuilder {
        <ListPingResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPingResponseBuilder {
    health_status: Option<String>,
}

impl ListPingResponseBuilder {
    pub fn health_status(mut self, value: impl Into<String>) -> Self {
        self.health_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPingResponse`].
    pub fn build(self) -> Result<ListPingResponse, BuildError> {
        Ok(ListPingResponse {
            health_status: self.health_status,
        })
    }
}
