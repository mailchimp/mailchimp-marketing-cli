pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportingFacebookAdAudienceTargetingSpecsInterestsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl ReportingFacebookAdAudienceTargetingSpecsInterestsItem {
    pub fn builder() -> ReportingFacebookAdAudienceTargetingSpecsInterestsItemBuilder {
        <ReportingFacebookAdAudienceTargetingSpecsInterestsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdAudienceTargetingSpecsInterestsItemBuilder {
    name: Option<String>,
}

impl ReportingFacebookAdAudienceTargetingSpecsInterestsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdAudienceTargetingSpecsInterestsItem`].
    pub fn build(self) -> Result<ReportingFacebookAdAudienceTargetingSpecsInterestsItem, BuildError> {
        Ok(ReportingFacebookAdAudienceTargetingSpecsInterestsItem {
            name: self.name,
        })
    }
}
