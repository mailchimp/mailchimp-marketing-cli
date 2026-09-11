pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CampaignsGetQueryRequest {
    /// A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub fields: Vec<Option<String>>,
    /// A comma-separated list of fields to exclude. Reference parameters of sub-objects with dot notation.
    #[serde(default)]
    pub exclude_fields: Vec<Option<String>>,
    /// Return the `resend_shortcut_eligibility` field in the response, which tells you if the campaign is eligible for the various Campaign Resend Shortcuts offered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resend_shortcut_eligibility: Option<bool>,
    /// Return the `resend_shortcut_usage` field in the response.  This includes information about campaigns related by a shortcut.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_resend_shortcut_usage: Option<bool>,
}

impl CampaignsGetQueryRequest {
    pub fn builder() -> CampaignsGetQueryRequestBuilder {
        <CampaignsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CampaignsGetQueryRequestBuilder {
    fields: Option<Vec<Option<String>>>,
    exclude_fields: Option<Vec<Option<String>>>,
    include_resend_shortcut_eligibility: Option<bool>,
    include_resend_shortcut_usage: Option<bool>,
}

impl CampaignsGetQueryRequestBuilder {
    pub fn fields(mut self, value: Vec<Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn exclude_fields(mut self, value: Vec<Option<String>>) -> Self {
        self.exclude_fields = Some(value);
        self
    }

    pub fn include_resend_shortcut_eligibility(mut self, value: bool) -> Self {
        self.include_resend_shortcut_eligibility = Some(value);
        self
    }

    pub fn include_resend_shortcut_usage(mut self, value: bool) -> Self {
        self.include_resend_shortcut_usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CampaignsGetQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](CampaignsGetQueryRequestBuilder::fields)
    /// - [`exclude_fields`](CampaignsGetQueryRequestBuilder::exclude_fields)
    pub fn build(self) -> Result<CampaignsGetQueryRequest, BuildError> {
        Ok(CampaignsGetQueryRequest {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            exclude_fields: self.exclude_fields.ok_or_else(|| BuildError::missing_field("exclude_fields"))?,
            include_resend_shortcut_eligibility: self.include_resend_shortcut_eligibility,
            include_resend_shortcut_usage: self.include_resend_shortcut_usage,
        })
    }
}

