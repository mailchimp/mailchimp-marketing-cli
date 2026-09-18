pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source from which the parent's entity was created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesContactSmsChannelMarketingConsentSource {
    /// The name of the entity's source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AudiencesContactSmsChannelMarketingConsentSource {
    pub fn builder() -> AudiencesContactSmsChannelMarketingConsentSourceBuilder {
        <AudiencesContactSmsChannelMarketingConsentSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactSmsChannelMarketingConsentSourceBuilder {
    name: Option<String>,
}

impl AudiencesContactSmsChannelMarketingConsentSourceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContactSmsChannelMarketingConsentSource`].
    pub fn build(self) -> Result<AudiencesContactSmsChannelMarketingConsentSource, BuildError> {
        Ok(AudiencesContactSmsChannelMarketingConsentSource {
            name: self.name,
        })
    }
}
