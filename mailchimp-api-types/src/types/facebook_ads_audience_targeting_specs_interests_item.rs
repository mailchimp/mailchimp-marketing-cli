pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FacebookAdsAudienceTargetingSpecsInterestsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl FacebookAdsAudienceTargetingSpecsInterestsItem {
    pub fn builder() -> FacebookAdsAudienceTargetingSpecsInterestsItemBuilder {
        <FacebookAdsAudienceTargetingSpecsInterestsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FacebookAdsAudienceTargetingSpecsInterestsItemBuilder {
    name: Option<String>,
}

impl FacebookAdsAudienceTargetingSpecsInterestsItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FacebookAdsAudienceTargetingSpecsInterestsItem`].
    pub fn build(self) -> Result<FacebookAdsAudienceTargetingSpecsInterestsItem, BuildError> {
        Ok(FacebookAdsAudienceTargetingSpecsInterestsItem {
            name: self.name,
        })
    }
}
