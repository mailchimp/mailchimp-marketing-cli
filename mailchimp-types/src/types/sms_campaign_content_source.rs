pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The source that created or imported this content.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SmsCampaignContentSource {
    /// The type of source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The ID of the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl SmsCampaignContentSource {
    pub fn builder() -> SmsCampaignContentSourceBuilder {
        <SmsCampaignContentSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SmsCampaignContentSourceBuilder {
    r#type: Option<String>,
    id: Option<String>,
}

impl SmsCampaignContentSourceBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SmsCampaignContentSource`].
    pub fn build(self) -> Result<SmsCampaignContentSource, BuildError> {
        Ok(SmsCampaignContentSource {
            r#type: self.r#type,
            id: self.id,
        })
    }
}
