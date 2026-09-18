pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source from which the parent's entity was created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchAudienceContactRequestSmsChannelMarketingConsentSource {
    /// The name of the entity's source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PatchAudienceContactRequestSmsChannelMarketingConsentSource {
    pub fn builder() -> PatchAudienceContactRequestSmsChannelMarketingConsentSourceBuilder {
        <PatchAudienceContactRequestSmsChannelMarketingConsentSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchAudienceContactRequestSmsChannelMarketingConsentSourceBuilder {
    name: Option<String>,
}

impl PatchAudienceContactRequestSmsChannelMarketingConsentSourceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PatchAudienceContactRequestSmsChannelMarketingConsentSource`].
    pub fn build(self) -> Result<PatchAudienceContactRequestSmsChannelMarketingConsentSource, BuildError> {
        Ok(PatchAudienceContactRequestSmsChannelMarketingConsentSource {
            name: self.name,
        })
    }
}
