pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReportingFacebookAdAudienceTargetingSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<ReportingFacebookAdAudienceTargetingSpecsInterestsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<ReportingFacebookAdAudienceTargetingSpecsLocations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_age: Option<i64>,
}

impl ReportingFacebookAdAudienceTargetingSpecs {
    pub fn builder() -> ReportingFacebookAdAudienceTargetingSpecsBuilder {
        <ReportingFacebookAdAudienceTargetingSpecsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportingFacebookAdAudienceTargetingSpecsBuilder {
    gender: Option<i64>,
    interests: Option<Vec<ReportingFacebookAdAudienceTargetingSpecsInterestsItem>>,
    locations: Option<ReportingFacebookAdAudienceTargetingSpecsLocations>,
    max_age: Option<i64>,
    min_age: Option<i64>,
}

impl ReportingFacebookAdAudienceTargetingSpecsBuilder {
    pub fn gender(mut self, value: i64) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn interests(mut self, value: Vec<ReportingFacebookAdAudienceTargetingSpecsInterestsItem>) -> Self {
        self.interests = Some(value);
        self
    }

    pub fn locations(mut self, value: ReportingFacebookAdAudienceTargetingSpecsLocations) -> Self {
        self.locations = Some(value);
        self
    }

    pub fn max_age(mut self, value: i64) -> Self {
        self.max_age = Some(value);
        self
    }

    pub fn min_age(mut self, value: i64) -> Self {
        self.min_age = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportingFacebookAdAudienceTargetingSpecs`].
    pub fn build(self) -> Result<ReportingFacebookAdAudienceTargetingSpecs, BuildError> {
        Ok(ReportingFacebookAdAudienceTargetingSpecs {
            gender: self.gender,
            interests: self.interests,
            locations: self.locations,
            max_age: self.max_age,
            min_age: self.min_age,
        })
    }
}
