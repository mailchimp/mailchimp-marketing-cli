pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source from which the parent's entity was created.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AudiencesContactEmailChannelSource {
    /// The name of the entity's source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AudiencesContactEmailChannelSource {
    pub fn builder() -> AudiencesContactEmailChannelSourceBuilder {
        <AudiencesContactEmailChannelSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudiencesContactEmailChannelSourceBuilder {
    name: Option<String>,
}

impl AudiencesContactEmailChannelSourceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AudiencesContactEmailChannelSource`].
    pub fn build(self) -> Result<AudiencesContactEmailChannelSource, BuildError> {
        Ok(AudiencesContactEmailChannelSource {
            name: self.name,
        })
    }
}
