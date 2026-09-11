pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get-content
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsGetContentQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
}

impl CampaignsGetContentQueryRequest {
    pub fn builder() -> CampaignsGetContentQueryRequestBuilder {
        <CampaignsGetContentQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsGetContentQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
}

impl CampaignsGetContentQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsGetContentQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](CampaignsGetContentQueryRequestBuilder::fields)
    /// - [`exclude_fields`](CampaignsGetContentQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<CampaignsGetContentQueryRequest, BuildError> {
        Ok(CampaignsGetContentQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
        })
    }
}

