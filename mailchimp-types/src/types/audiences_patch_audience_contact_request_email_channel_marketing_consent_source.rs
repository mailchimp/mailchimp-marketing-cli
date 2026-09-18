pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source from which the parent's entity was created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchAudienceContactRequestEmailChannelMarketingConsentSource {
    /// The name of the entity's source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchAudienceContactRequestEmailChannelMarketingConsentSource {
    pub fn builder() -> PatchAudienceContactRequestEmailChannelMarketingConsentSourceBuilder {
        <PatchAudienceContactRequestEmailChannelMarketingConsentSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchAudienceContactRequestEmailChannelMarketingConsentSourceBuilder {
    name: Option<String>,
}

impl PatchAudienceContactRequestEmailChannelMarketingConsentSourceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchAudienceContactRequestEmailChannelMarketingConsentSource`].
    pub fn build(self) -> Result<PatchAudienceContactRequestEmailChannelMarketingConsentSource, BuildError> {
        Ok(PatchAudienceContactRequestEmailChannelMarketingConsentSource {
            name: self.name,
        })
    }
}
