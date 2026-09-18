pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source from which the parent's entity was created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesContactSmsChannelSource {
    /// The name of the entity's source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AudiencesContactSmsChannelSource {
    pub fn builder() -> AudiencesContactSmsChannelSourceBuilder {
        <AudiencesContactSmsChannelSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactSmsChannelSourceBuilder {
    name: Option<String>,
}

impl AudiencesContactSmsChannelSourceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContactSmsChannelSource`].
    pub fn build(self) -> Result<AudiencesContactSmsChannelSource, BuildError> {
        Ok(AudiencesContactSmsChannelSource {
            name: self.name,
        })
    }
}
