pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Audience settings
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportingFacebookAdAudience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_source: Option<ReportingFacebookAdAudienceEmailSource>,
    /// To include list contacts as part of audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_source_in_target: Option<bool>,
    /// To find similar audience in given country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookalike_country_code: Option<String>,
    /// List or Facebook based audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<ReportingFacebookAdAudienceSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targeting_specs: Option<ReportingFacebookAdAudienceTargetingSpecs>,
    /// Type of the audience
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ReportingFacebookAdAudienceType>,
}

impl ReportingFacebookAdAudience {
    pub fn builder() -> ReportingFacebookAdAudienceBuilder {
        <ReportingFacebookAdAudienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdAudienceBuilder {
    email_source: Option<ReportingFacebookAdAudienceEmailSource>,
    include_source_in_target: Option<bool>,
    lookalike_country_code: Option<String>,
    source_type: Option<ReportingFacebookAdAudienceSourceType>,
    targeting_specs: Option<ReportingFacebookAdAudienceTargetingSpecs>,
    r#type: Option<ReportingFacebookAdAudienceType>,
}

impl ReportingFacebookAdAudienceBuilder {
    pub fn email_source(mut self, value: ReportingFacebookAdAudienceEmailSource) -> Self {
        self.email_source = Some(value);
        self
    }

    pub fn include_source_in_target(mut self, value: bool) -> Self {
        self.include_source_in_target = Some(value);
        self
    }

    pub fn lookalike_country_code(mut self, value: impl Into<String>) -> Self {
        self.lookalike_country_code = Some(value.into());
        self
    }

    pub fn source_type(mut self, value: ReportingFacebookAdAudienceSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn targeting_specs(mut self, value: ReportingFacebookAdAudienceTargetingSpecs) -> Self {
        self.targeting_specs = Some(value);
        self
    }

    pub fn r#type(mut self, value: ReportingFacebookAdAudienceType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdAudience`].
    pub fn build(self) -> Result<ReportingFacebookAdAudience, BuildError> {
        Ok(ReportingFacebookAdAudience {
            email_source: self.email_source,
            include_source_in_target: self.include_source_in_target,
            lookalike_country_code: self.lookalike_country_code,
            source_type: self.source_type,
            targeting_specs: self.targeting_specs,
            r#type: self.r#type,
        })
    }
}
