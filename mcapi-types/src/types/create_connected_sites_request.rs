pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConnectedSitesRequest {
    /// The connected site domain.
    #[serde(default)]
    pub domain: String,
    /// The unique identifier for the site.
    #[serde(default)]
    pub foreign_id: String,
}

impl CreateConnectedSitesRequest {
    pub fn builder() -> CreateConnectedSitesRequestBuilder {
        <CreateConnectedSitesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConnectedSitesRequestBuilder {
    domain: Option<String>,
    foreign_id: Option<String>,
}

impl CreateConnectedSitesRequestBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn foreign_id(mut self, value: impl Into<String>) -> Self {
        self.foreign_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConnectedSitesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](CreateConnectedSitesRequestBuilder::domain)
    /// - [`foreign_id`](CreateConnectedSitesRequestBuilder::foreign_id)
    pub fn build(self) -> Result<CreateConnectedSitesRequest, BuildError> {
        Ok(CreateConnectedSitesRequest {
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
            foreign_id: self.foreign_id.ok_or_else(|| BuildError::missing_field("foreign_id"))?,
        })
    }
}

