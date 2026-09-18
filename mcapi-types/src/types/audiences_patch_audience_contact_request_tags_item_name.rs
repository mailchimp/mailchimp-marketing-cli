pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PatchAudienceContactRequestTagsItemName {
    #[serde(default)]
    pub name: String,
    pub status: PatchAudienceContactRequestTagsItemNameStatus,
}

impl PatchAudienceContactRequestTagsItemName {
    pub fn builder() -> PatchAudienceContactRequestTagsItemNameBuilder {
        <PatchAudienceContactRequestTagsItemNameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchAudienceContactRequestTagsItemNameBuilder {
    name: Option<String>,
    status: Option<PatchAudienceContactRequestTagsItemNameStatus>,
}

impl PatchAudienceContactRequestTagsItemNameBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn status(mut self, value: PatchAudienceContactRequestTagsItemNameStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchAudienceContactRequestTagsItemName`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PatchAudienceContactRequestTagsItemNameBuilder::name)
    /// - [`status`](PatchAudienceContactRequestTagsItemNameBuilder::status)
    pub fn build(self) -> Result<PatchAudienceContactRequestTagsItemName, BuildError> {
        Ok(PatchAudienceContactRequestTagsItemName {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
