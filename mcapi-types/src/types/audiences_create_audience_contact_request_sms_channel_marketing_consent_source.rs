pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source from which the parent's entity was created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAudienceContactRequestSmsChannelMarketingConsentSource {
    /// The name of the entity's source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateAudienceContactRequestSmsChannelMarketingConsentSource {
    pub fn builder() -> CreateAudienceContactRequestSmsChannelMarketingConsentSourceBuilder {
        <CreateAudienceContactRequestSmsChannelMarketingConsentSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudienceContactRequestSmsChannelMarketingConsentSourceBuilder {
    name: Option<String>,
}

impl CreateAudienceContactRequestSmsChannelMarketingConsentSourceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAudienceContactRequestSmsChannelMarketingConsentSource`].
    pub fn build(self) -> Result<CreateAudienceContactRequestSmsChannelMarketingConsentSource, BuildError> {
        Ok(CreateAudienceContactRequestSmsChannelMarketingConsentSource {
            name: self.name,
        })
    }
}
